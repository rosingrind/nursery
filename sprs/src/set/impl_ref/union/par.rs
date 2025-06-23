use num_traits::{AsPrimitive, Unsigned};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator, plumbing::UnindexedConsumer};

use crate::set::{SetRef, SparSet};

pub struct Union<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) a: &'a SparSet<K>,
    pub(in crate::set) b: &'a SparSet<K>,
}

impl<'a, K> ParallelIterator for Union<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Send + Sync,
{
    type Item = &'a K;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        // We'll iterate one set in full, and only the remaining difference from the other.
        // Use the smaller set for the difference in order to reduce hash lookups.
        let (smaller, larger) = if self.a.len() <= self.b.len() {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        };
        larger
            .into_par_iter()
            .chain(smaller.difference(larger))
            .drive_unindexed(consumer)
    }
}
