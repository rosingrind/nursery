use std::mem::MaybeUninit;

#[cfg_attr(feature = "rayon", allow(unused_imports))]
use itertools::Itertools;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{BeamError, Node};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct MockNode<const THRESHOLD: usize> {
    pub(crate) count: usize,
}

impl<const THRESHOLD: usize, const B: usize> Node<B> for MockNode<THRESHOLD> {
    #[cfg(not(feature = "rayon"))]
    fn expand<'a, I: Iterator<Item = &'a mut MaybeUninit<Self>>>(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError> {
        let fill_op = |(i, x): (usize, &mut MaybeUninit<Self>)| {
            self.count.checked_add(i + 1).map(|count| {
                x.write(Self { count });
            })
        };

        Ok(iter.enumerate().map(fill_op).while_some().count())
    }

    #[cfg(feature = "rayon")]
    fn expand<
        'a,
        I: ParallelIterator<Item = &'a mut MaybeUninit<Self>> + IndexedParallelIterator,
    >(
        &'a self,
        iter: I,
    ) -> Result<usize, BeamError> {
        let fill_op = |(i, x): (usize, &mut MaybeUninit<Self>)| {
            self.count.checked_add(i + 1).map(|count| {
                x.write(Self { count });
            })
        };

        Ok(iter.enumerate().map(fill_op).while_some().count())
    }

    fn evaluate(&self) -> u64 {
        const ACCURACY: f64 = 10_000.0;
        (ACCURACY / self.count as f64).round() as u64
    }

    fn has_fulfilled(&self) -> bool {
        self.count >= THRESHOLD
    }
}
