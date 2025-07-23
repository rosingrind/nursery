#[cfg(feature = "test-utils")]
mod common;

#[cfg(feature = "test-utils")]
use bspa::Node;
use bspa::{Area, Beam, BspaNode, Placement, Rect, RectGroup};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

const BW: usize = 150_000;
const BB: usize = 15_000;
const N: usize = 250_000;
const F: f32 = 1.0;

#[test]
fn simple_atlas_packing() {
    const RECT_S: Rect = Rect::new(8, 8);
    const RECT_L: Rect = Rect::new(16, 16);

    const ITEMS: [Rect; 5] = [RECT_S, RECT_S, RECT_S, RECT_S, RECT_L];

    let node = BspaNode::new(ITEMS, 32, N, F);
    let mut beam: Beam<BW, BB, _> = node.into();

    while !(matches!(beam.cycle(), Err(beam::BeamError::Exhausted)) || beam.has_fulfilled()) {}

    #[cfg(feature = "test-utils")]
    common::save_pg(
        &ITEMS.iter().copied().collect(),
        beam.nodes()
            .min_by_key(|n| Node::<BB>::evaluate(*n))
            .unwrap()
            .blocks(),
        "simple_blocks.png",
    );

    let data = [
        Placement {
            x: 0,
            y: 0,
            item: RECT_L,
        },
        Placement {
            x: 16,
            y: 0,
            item: RECT_S,
        },
        Placement {
            x: 16,
            y: 8,
            item: RECT_S,
        },
        Placement {
            x: 24,
            y: 0,
            item: RECT_S,
        },
        Placement {
            x: 24,
            y: 8,
            item: RECT_S,
        },
    ]
    .into_iter()
    .collect::<RectGroup>();

    assert!(
        beam.nodes().any(|x| {
            #[cfg(not(feature = "rayon"))]
            let iter = x.blocks().to_owned().into_iter();
            #[cfg(feature = "rayon")]
            let iter = x.blocks().to_owned().into_par_iter();

            iter.flat_map(|b| b.into_placed_rects())
                .collect::<RectGroup>()
                .area()
                == data.area()
        }),
        "beam.nodes().blocks: {:#?}",
        beam.nodes()
            .map(|x| {
                #[cfg(not(feature = "rayon"))]
                let iter = x.blocks().to_owned().into_iter();
                #[cfg(feature = "rayon")]
                let iter = x.blocks().to_owned().into_par_iter();

                let mut c = iter.flat_map(|b| b.into_placed_rects()).collect::<Vec<_>>();
                c.sort_by_key(|k| (k.x as u64) | ((k.y as u64) << 8));
                c
            })
            .collect::<Vec<_>>()
    );
}

#[test]
fn varied_atlas_packing() {
    const ITEMS: [Rect; 16] = [
        Rect::new(12, 8),
        Rect::new(8, 4),
        Rect::new(10, 10),
        Rect::new(12, 8),
        Rect::new(15, 15),
        Rect::new(15, 15),
        Rect::new(16, 12),
        Rect::new(8, 18),
        Rect::new(8, 12),
        Rect::new(7, 11),
        Rect::new(13, 6),
        Rect::new(14, 14),
        Rect::new(4, 19),
        Rect::new(2, 10),
        Rect::new(7, 16),
        Rect::new(11, 9),
    ];

    let node = BspaNode::new(
        ITEMS,
        ITEMS.into_iter().map(|x| x.w()).max().unwrap() * 2,
        N,
        F,
    );
    let mut beam: Beam<BW, BB, _> = node.into();

    while !(matches!(beam.cycle(), Err(beam::BeamError::Exhausted)) || beam.has_fulfilled()) {}

    beam.extend();
    while !beam.has_fulfilled() {
        beam.cycle().unwrap();
    }

    #[cfg(feature = "test-utils")]
    common::save_pg(
        &ITEMS.iter().copied().collect(),
        beam.nodes()
            .min_by_key(|n| Node::<BB>::evaluate(*n))
            .unwrap()
            .blocks(),
        "varied_blocks.png",
    );

    assert!(
        beam.nodes().any(|x| {
            #[cfg(not(feature = "rayon"))]
            let iter = x.blocks().to_owned().into_iter();
            #[cfg(feature = "rayon")]
            let iter = x.blocks().to_owned().into_par_iter();

            iter.flat_map(|b| b.into_placed_rects()).count() == ITEMS.len()
        }),
        "beam.nodes().blocks: {:#?}",
        beam.nodes()
            .filter_map(|x| {
                #[cfg(not(feature = "rayon"))]
                let iter = x.blocks().to_owned().into_iter();
                #[cfg(feature = "rayon")]
                let iter = x.blocks().to_owned().into_par_iter();

                let mut c = iter.flat_map(|b| b.into_placed_rects()).collect::<Vec<_>>();
                c.sort_by_key(|k| (k.x as u64) | ((k.y as u64) << 8));
                (!c.is_empty()).then_some(c)
            })
            .collect::<Vec<_>>()
    );
}
