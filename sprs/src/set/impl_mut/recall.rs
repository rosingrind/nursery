use num_traits::{AsPrimitive, Unsigned};
use std::iter;

use crate::set::{SetRef, SparSet};

pub(in crate::set) struct RawRecall<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) pos: usize,
    pub(in crate::set) table: &'a mut SparSet<K>,
}

impl<K> RawRecall<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn next<F>(&mut self, f: F) -> Option<K>
    where
        F: Fn(&K) -> bool,
    {
        while likely_stable::likely(self.pos < self.table.len().as_()) {
            let item = self.table.as_slice()[self.pos];
            let cond = f(&item);
            self.pos += std::hint::select_unpredictable(cond, 0, 1);
            if cond {
                let old = Some(item);
                self.table.delete_one_seq_uncheck(item);
                return old;
            }
        }
        None
    }
}

pub struct Recall<'a, K, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K) -> bool,
{
    pub(in crate::set) f: F,
    pub(in crate::set) inner: RawRecall<'a, K>,
}

impl<K, F> Iterator for Recall<'_, K, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K) -> bool,
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|k| (self.f)(k))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.table.len().as_() - self.inner.pos;
        (len, Some(len))
    }
}

impl<K, F> iter::FusedIterator for Recall<'_, K, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K) -> bool,
{
}

impl<K, F> iter::ExactSizeIterator for Recall<'_, K, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K) -> bool,
{
}
