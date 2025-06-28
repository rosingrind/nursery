#[cfg(debug_assertions)]
mod assert;
#[cfg(test)]
mod tests;

use std::mem::{self, MaybeUninit};

use beam::{BeamError, Node};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use crate::{Area, BspaNode, types::*};

impl<const B: usize> Node<B> for BspaNode {
    fn has_fulfilled(&self) -> bool {
        self != &Self::default() && self.avai_box.values().sum::<usize>() == 0
    }

    #[cfg(not(feature = "rayon"))]
    fn expand<'a, I: Iterator<Item = &'a mut MaybeUninit<Self>>>(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError> {
        use crate::RectGroup;

        let (space, block_pool) = self.prepare::<B>()?;

        let fill_op = |(x, block): (&mut MaybeUninit<Self>, RectGroup)| {
            let _x = self.advance(x, space, block);

            #[cfg(debug_assertions)]
            assert::assert_node_expand(_x);
        };

        Ok(iter.zip(block_pool).map(fill_op).count())
    }

    #[cfg(feature = "rayon")]
    fn expand<'a, I: IndexedParallelIterator<Item = &'a mut MaybeUninit<Self>>>(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError> {
        let (space, block_pool) = self.prepare::<B>()?;

        let fill_op = |(x, block): (&mut MaybeUninit<Self>, RectGroup)| {
            let x = self.advance(x, space, block);

            #[cfg(debug_assertions)]
            assert::assert_node_expand(x);
        };

        Ok(iter.zip(block_pool).map(fill_op).count())
    }

    fn evaluate(&self) -> u64 {
        #[inline]
        fn avg_high(avai_box: &FxHashMap<Rect, usize>) -> f64 {
            let (s, l) = avai_box
                .iter()
                .fold((0u64, 0usize), |(mut s, mut l), (k, &v)| {
                    let d = v;
                    l += d;
                    s += k.h() as u64 * d as u64;
                    (s, l)
                });
            s as f64 / l as f64
        }

        #[inline]
        fn cast<T>(value: T) -> u32
        where
            u32: TryFrom<T>,
            <u32 as TryFrom<T>>::Error: std::fmt::Debug,
        {
            u32::try_from(value).unwrap()
        }

        let heuristic =
            self.avai_box.values().sum::<usize>() as u64 + avg_high(&self.avai_box).round() as u64;
        (cast(heuristic) as u64) << u32::BITS | cast(self.area() - self.fill_area()) as u64
    }

    fn inflate(&mut self) {
        if self == &Self::default() {
            return;
        }

        let xmax = self
            .spaces
            .iter()
            .map(|s| s.x + s.w())
            .max()
            .unwrap_or_else(|| self.w());
        let ymax = self.h();

        let mut lhs = &mut self.clone();

        lhs.spaces
            .iter_mut()
            .filter(|s| s.y + s.h() >= ymax)
            .for_each(|s| s.item = Rect::new(s.w(), u32::MAX - s.y));

        if !lhs
            .spaces
            .iter()
            .filter(|s| s.y + s.h() >= ymax)
            .any(|s| s.w() >= xmax)
        {
            // full width space
            lhs.spaces.reserve_exact(1);
            lhs.spaces.push(Placement {
                x: 0,
                y: ymax,
                item: Rect::new(xmax, u32::MAX - ymax),
            });
        }

        let mut rhs = &mut Self::default();
        let top = loop {
            let Ok((space, mut block_pool)) = lhs.prepare::<1>() else {
                break lhs;
            };
            let block = block_pool.pop().unwrap();

            lhs.advance(
                unsafe { mem::transmute::<&mut _, &mut MaybeUninit<_>>(&mut *rhs) },
                space,
                block,
            );
            std::mem::swap(&mut lhs, &mut rhs);
        };
        let d = top.h() - self.h();
        let _ = rhs;
        let _ = top;

        self.spaces
            .iter_mut()
            .filter(|s| s.y + s.h() >= ymax)
            .for_each(|s| s.item = Rect::new(s.w(), ymax + d - s.y));

        if !self
            .spaces
            .iter()
            .filter(|s| s.y + s.h() >= ymax)
            .any(|s| s.w() >= xmax)
        {
            // full width space
            self.spaces.reserve_exact(1);
            self.spaces.push(Placement {
                x: 0,
                y: ymax,
                item: Rect::new(xmax, d),
            });
        }

        #[cfg(debug_assertions)]
        assert::assert_node_inflate(self);
    }

    fn estimate(&self) -> Option<usize> {
        self.prepare::<B>()
            .ok()
            .map(|(_, block_pool)| block_pool.len())
    }
}
