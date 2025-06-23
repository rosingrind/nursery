use num_traits::{AsPrimitive, Unsigned};

#[cfg(feature = "rayon")]
use rayon::iter::{IntoParallelIterator, ParallelIterator, plumbing::UnindexedConsumer};

use crate::set::SparSet;

pub struct Difference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) a: &'a SparSet<K>,
    pub(in crate::set) b: &'a SparSet<K>,
}

impl<'a, K> ParallelIterator for Difference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Send + Sync,
{
    type Item = &'a K;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.a
            .into_par_iter()
            .filter(|&x| !self.b.contains(*x))
            .drive_unindexed(consumer)
    }
}
