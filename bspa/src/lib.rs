mod area;
mod node;
mod types;

use std::mem::MaybeUninit;

use itertools::Itertools;
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use rustc_hash::FxHashMap;

pub use beamsrch::*;
pub use types::*;

pub trait Area {
    /// Occupied area (w * h)
    fn area(&self) -> u64;

    /// Area filled inside container
    fn fill_area(&self) -> u64;

    /// Get `w` of container
    fn w(&self) -> u32;

    /// Get `h` of container
    fn h(&self) -> u32;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BspaNode {
    /// Spaces available to place blocks
    spaces: Vec<Placement<Rect>>,
    /// Blocks placed to spaces
    blocks: Vec<Placement<RectGroup>>,
    /// Not yet combined boxes
    avai_box: FxHashMap<Rect, usize>,
    /// Not yet placed available blocks from combined boxes
    avai_blk: Vec<RectGroup>,
}

impl BspaNode {
    pub fn new<I: IntoIterator<Item = Rect>>(avai_box: I, w: u32, n: usize, f: f32) -> Self {
        let mut avai_box = avai_box
            .into_iter()
            .fold(FxHashMap::default(), |mut acc, c| {
                *acc.entry(c).or_default() += 1;
                acc
            });
        avai_box.shrink_to_fit();

        let s = avai_box
            .iter()
            .map(|(k, v)| k.area() * *v as u64)
            .sum::<u64>();
        let h = (s / w as u64) as u32;

        let avai_blk = avai_box
            .clone()
            .into_iter()
            .flat_map(|(bt, bc)| {
                (1..=bc).flat_map(move |w| {
                    (1..=(bc / w)).map(move |l| {
                        (0..w)
                            .cartesian_product(0..l)
                            .map(move |(x, y)| Placement {
                                x: x as u32 * bt.w(),
                                y: y as u32 * bt.h(),
                                item: bt,
                            })
                            .collect::<RectGroup>()
                    })
                })
            })
            .filter(|b| {
                let fill_rate = b.fill_area() as f32 / b.area() as f32;
                fill_rate >= f
            })
            .collect::<Vec<_>>();

        let mut avai_blk = avai_blk
            .clone()
            .into_iter()
            .cartesian_product(avai_blk.clone())
            .flat_map(|(a, b)| {
                a.combine(b).into_iter().filter(|b| {
                    let boxes = b.list.iter().fold(FxHashMap::default(), |mut acc, c| {
                        *acc.entry(c.item).or_default() += 1;
                        acc
                    });
                    (b.w() <= w)
                        & (b.h() <= h)
                        & (avai_box
                            .iter()
                            .all(|(k, v)| v >= boxes.get(k).unwrap_or(&0)))
                        & (b.fill_area() as f32 / b.area() as f32 >= f)
                })
            })
            .take(n)
            .chain(avai_blk)
            .collect::<Vec<_>>();
        avai_blk.shrink_to_fit();

        let space = Rect::new(w, h);
        Self {
            spaces: From::from([Placement {
                x: 0,
                y: 0,
                item: space,
            }]),
            blocks: Default::default(),
            avai_box,
            avai_blk,
        }
    }

    pub fn blocks(&self) -> &[Placement<RectGroup>] {
        &self.blocks
    }

    #[inline]
    fn sel_space(&self) -> impl Iterator<Item = Placement<Rect>> + use<'_> {
        let mut buf = self.spaces.iter().collect::<Box<_>>();
        buf.sort();
        buf.into_iter().copied()
    }

    #[inline]
    fn sel_block<'a, const B: usize>(
        &'a self,
        space: &Placement<Rect>,
    ) -> impl Iterator<Item = RectGroup> + use<'a, B> {
        #[inline]
        fn avg_high(avai_box: &FxHashMap<Rect, usize>, block: &RectGroup) -> f64 {
            let (s, l) = avai_box
                .iter()
                .fold((0u64, 0usize), |(mut s, mut l), (k, &v)| {
                    let count = block.list.iter().filter(|&p| &p.item == k).count();
                    let d = v - count;
                    l += d;
                    s += k.h() as u64 * d as u64;
                    (s, l)
                });
            s as f64 / l as f64
        }

        let mut vec = self
            .avai_blk
            .iter()
            .filter(|b| (space.item.w() >= b.w()) & (space.item.h() >= b.h()))
            .collect::<Box<_>>();
        #[cfg(not(feature = "rayon"))]
        vec.sort_unstable_by_key(|b| b.score(space, avg_high(&self.avai_box, b)));
        #[cfg(feature = "rayon")]
        vec.par_sort_unstable_by_key(|b| b.score(space, avg_high(&self.avai_box, b)));
        vec.into_iter().take(B).cloned()
    }

    #[inline]
    fn gen_space<'a>(
        &'a self,
        block: &'a Placement<RectGroup>,
    ) -> impl Iterator<Item = Placement<Rect>> + use<'a> {
        let l = self
            .spaces
            .iter()
            .filter(|s| s.overlaps(block))
            .flat_map(|s| s.substract(block));
        let r = self.spaces.iter().filter(|s| !s.overlaps(block)).copied();
        l.chain(r)
    }

    #[inline]
    fn prepare<const B: usize>(&self) -> Result<(Placement<Rect>, Vec<RectGroup>), BeamError> {
        let mut spaces = self.sel_space();

        loop {
            let space = spaces.next().ok_or(BeamError::BranchExhausted)?;
            let block = self.sel_block::<B>(&space).collect::<Vec<_>>();

            if !block.is_empty() {
                break Ok((space, block));
            }
        }
    }

    fn advance<'a>(
        &self,
        other: &'a mut MaybeUninit<Self>,
        space: Placement<Rect>,
        block: RectGroup,
    ) -> &'a mut Self {
        let mut avai_box = self.avai_box.clone();
        avai_box
            .iter_mut()
            .for_each(|(k, v)| *v -= block.list.iter().filter(|&p| &p.item == k).count());

        let avai_blk = self
            .avai_blk
            .iter()
            .filter(|b| {
                avai_box
                    .iter()
                    .all(|(k, v)| *v >= b.list.iter().filter(|p| &p.item == k).count())
            })
            .cloned()
            .collect::<Vec<_>>();

        let blocks = self
            .blocks
            .iter()
            .cloned()
            .chain(std::iter::once(Placement {
                x: space.x,
                y: space.y,
                item: block,
            }))
            .collect::<Vec<_>>();

        let spaces = self.gen_space(blocks.last().unwrap()).collect::<Vec<_>>();

        other.write(Self {
            spaces,
            blocks,
            avai_box,
            avai_blk,
        })
    }
}
