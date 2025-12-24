#[path = "../src/tests/mock.rs"]
mod mock;

use std::hint::black_box;

use beamsrch::{Beam, BeamError, Node};
use bencher::Bencher;

const BW: usize = 50;
const BB: usize = 50;
const TH: usize = 10_000;

fn fulfillment(b: &mut Bencher) {
    let node = black_box(mock::MockNode::<TH>::default());

    b.iter(|| {
        let mut beam: Beam<BW, BB, _> = node.into();
        while !black_box(beam.has_fulfilled()) && black_box(beam.cycle()).is_ok() {}
    });
}

fn single_iter(b: &mut Bencher) {
    let node = black_box(mock::MockNode::<TH>::default());

    b.iter(|| {
        let mut beam: Beam<BW, BB, _> = node.into();
        let _ = black_box(beam.cycle());
    });
}

bencher::benchmark_group!(benches, fulfillment, single_iter);
bencher::benchmark_main!(benches);
