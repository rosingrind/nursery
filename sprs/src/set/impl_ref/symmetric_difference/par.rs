use num_traits::{AsPrimitive, Unsigned};

#[cfg(feature = "rayon")]
use rayon::iter::{ParallelIterator, plumbing::UnindexedConsumer};

use crate::set::{SetRef, SparSet};

pub struct SymmetricDifference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) a: &'a SparSet<K>,
    pub(in crate::set) b: &'a SparSet<K>,
}

impl<'a, K> ParallelIterator for SymmetricDifference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Send + Sync,
{
    type Item = &'a K;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.a
            .difference(self.b)
            .chain(self.b.difference(self.a))
            .drive_unindexed(consumer)
    }
}
