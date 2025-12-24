mod mock;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{Beam, Node};

#[test]
fn simple_cycle() {
    const BW: usize = 1;
    const BB: usize = 1;
    const TH: usize = 1_000;

    let node = mock::MockNode::<TH>::default();
    let mut beam: Beam<BW, BB, _> = node.into();

    assert_eq!(
        unsafe { beam.node_buf.first().unwrap().assume_init_ref().count },
        1
    );

    beam.cycle().unwrap();

    assert_eq!(
        unsafe { beam.node_buf.first().unwrap().assume_init_ref().count },
        2
    );

    while !beam.has_fulfilled() {
        beam.cycle().unwrap()
    }

    assert_eq!(
        beam.nodes()
            .min_by_key(|node| <mock::MockNode<TH> as Node<BB>>::evaluate(*node))
            .unwrap()
            .count,
        TH
    );
}

#[test]
fn varied_cycle() {
    const BW: usize = 75;
    const BB: usize = 75;
    const TH: usize = 1_425;

    let node = mock::MockNode::<TH>::default();
    let mut beam: Beam<BW, BB, _> = node.into();

    assert_eq!(
        unsafe { beam.node_buf.first().unwrap().assume_init_ref().count },
        1
    );

    beam.cycle().unwrap();

    assert_eq!(
        unsafe { beam.node_buf.first().unwrap().assume_init_ref().count },
        76
    );

    while !beam.has_fulfilled() {
        beam.cycle().unwrap()
    }

    assert_eq!(
        beam.nodes()
            .min_by_key(|node| <mock::MockNode<TH> as Node<BB>>::evaluate(*node))
            .unwrap()
            .count,
        TH
    );
}
