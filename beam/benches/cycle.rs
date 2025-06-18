#[path = "../src/tests/mock.rs"]
mod mock;

use std::time::Duration;

use beam::{Beam, BeamError, Node};
use divan::{Bencher, black_box};

const BW: usize = 50;
const BB: usize = 50;
const TH: usize = 10_000;

#[divan::bench(max_time = Duration::from_secs(3))]
fn single_iter(bencher: Bencher) {
    let node = mock::MockNode::<TH>::default();

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.into());
        let _ = black_box(beam.cycle());
    });
}

#[divan::bench(max_time = Duration::from_secs(3))]
fn fulfillment(bencher: Bencher) {
    let node = mock::MockNode::<TH>::default();

    bencher.bench(|| {
        let mut beam: Beam<BW, BB, _> = black_box(node.into());
        while black_box(!beam.has_fulfilled()) {
            let _ = black_box(beam.cycle());
        }
    });
}

fn main() {
    divan::main();
}
