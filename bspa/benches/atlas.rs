use std::time::Duration;

use beam::Beam;
use bspa::{Area, BspaNode, Rect};
use divan::{Bencher, black_box};

const N: usize = 100_000;
const F: f32 = 1.0;
const BW: usize = 1;
const BB: usize = 1;

#[divan::bench(max_time = Duration::from_secs(3))]
fn simple_atlas_packing(bencher: Bencher) {
    const RECT_S: Rect = Rect::new(8, 8);
    const RECT_L: Rect = Rect::new(16, 16);

    const ITEMS: [Rect; 5] = [RECT_S, RECT_S, RECT_S, RECT_S, RECT_L];

    let node = BspaNode::new(ITEMS, 32, N, F);

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.clone().into());
        while black_box(!matches!(
            black_box(beam.cycle()),
            Err(beam::BeamError::Exhausted)
        )) {}
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn varied_atlas_packing(bencher: Bencher) {
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

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.clone().into());
        while black_box(!matches!(
            black_box(beam.cycle()),
            Err(beam::BeamError::Exhausted)
        )) {}
        beam.extend();
        while !black_box(beam.has_fulfilled()) {
            black_box(beam.cycle()).unwrap();
        }
    });
}

fn main() {
    divan::main();
}
