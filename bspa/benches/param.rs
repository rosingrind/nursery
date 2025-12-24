use std::hint::black_box;

use beamsrch::Beam;
use bencher::Bencher;
use bspa::{BspaNode, Rect};

const N: usize = 100_000;
const F: f32 = 1.0;

const RECT_S: Rect = Rect::new(8, 8);
const RECT_L: Rect = Rect::new(16, 16);

const ITEMS: [Rect; 5] = [RECT_S, RECT_S, RECT_S, RECT_S, RECT_L];

#[allow(non_snake_case)]
fn bw_bb<const BW: usize, const BB: usize>(b: &mut Bencher) {
    let node = BspaNode::new(ITEMS, 32, N, F);

    b.iter(|| {
        let mut beam: Beam<BW, BB, _> = node.clone().into();
        let _ = black_box(beam.cycle());
    });
}

bencher::benchmark_group!(
    benches,
    bw_bb<10_000, 10_000>,
    bw_bb<10_000, 35_000>,
    bw_bb<10_000, 97_500>,
    bw_bb<35_000, 10_000>,
    bw_bb<97_500, 10_000>
);
bencher::benchmark_main!(benches);
