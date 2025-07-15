use num_traits::{AsPrimitive, Unsigned};
use std::iter;

use crate::set::{SetMut, SparSet};

pub(in crate::set) struct RawRecall<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) iter: std::vec::IntoIter<K>,
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
        for item in self.iter.by_ref() {
            if f(&item) {
                let old = self.table.contains(item).then_some(item);
                self.table.delete_one(item);
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
        (0, self.inner.iter.size_hint().1)
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
