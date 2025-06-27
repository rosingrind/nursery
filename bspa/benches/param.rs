use std::hint::black_box;

use beam::Beam;
use bencher::Bencher;
use bspa::{BspaNode, Rect};

const N: usize = 100_000;
const F: f32 = 1.0;

const RECT_S: Rect = Rect::new(8, 8);
const RECT_L: Rect = Rect::new(16, 16);

const ITEMS: [Rect; 5] = [RECT_S, RECT_S, RECT_S, RECT_S, RECT_L];

#[allow(non_snake_case)]
fn cond<const BW: usize, const BB: usize>(b: &mut Bencher) {
    let node = BspaNode::new(ITEMS, 32, N, F);

    b.iter(|| {
        let mut beam: Beam<BW, BB, _> = node.clone().into();
        let _ = black_box(beam.cycle());
    });
}

bencher::benchmark_group!(
    benches,
    cond<10_000, 10_000>,
    cond<10_000, 20_000>,
    cond<10_000, 50_000>,
    cond<20_000, 10_000>,
    cond<50_000, 10_000>
);
bencher::benchmark_main!(benches);
