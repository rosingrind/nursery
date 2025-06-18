use std::time::Duration;

use beam::Beam;
use bspa::{BspaNode, Rect};
use divan::{Bencher, black_box};

const N: usize = 100_000;
const F: f32 = 1.0;

const RECT_S: Rect = Rect::new(8, 8);
const RECT_L: Rect = Rect::new(16, 16);

const ITEMS: [Rect; 5] = [RECT_S, RECT_S, RECT_S, RECT_S, RECT_L];

#[allow(non_snake_case)]
#[divan::bench(consts = [100_000, 200_000, 500_000], max_time = Duration::from_secs(3))]
fn BB<const BB: usize>(bencher: Bencher) {
    const BW: usize = 100;

    let node = BspaNode::new(ITEMS, 32, N, F);

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.clone().into());
        let _ = black_box(beam.cycle());
    });
}

#[allow(non_snake_case)]
#[divan::bench(consts = [100_000, 200_000, 500_000], max_time = Duration::from_secs(3))]
fn BW<const BW: usize>(bencher: Bencher) {
    const BB: usize = 100;

    let node = BspaNode::new(ITEMS, 32, N, F);

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.clone().into());
        let _ = black_box(beam.cycle());
    });
}

fn main() {
    divan::main();
}
