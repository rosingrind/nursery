use std::iter;

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SparSet};

pub(super) struct RawRecall<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: std::vec::IntoIter<K>,
    pub(super) table: &'a mut SparSet<K>,
}

impl<K> RawRecall<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
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
    pub(super) f: F,
    pub(super) inner: RawRecall<'a, K>,
}

impl<K, F> Iterator for Recall<'_, K, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K) -> bool,
{
    type Item = K;

    #[cfg_attr(feature = "inline-more", inline)]
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
