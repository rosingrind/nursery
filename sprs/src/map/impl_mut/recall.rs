use std::iter;

use num_traits::{AsPrimitive, Unsigned};

use crate::map::{MapMut, SparMap};

pub(super) struct RawRecall<'a, K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: std::vec::IntoIter<(K, V)>,
    pub(super) table: &'a mut SparMap<K, V>,
}

impl<K, V> RawRecall<'_, K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn next<F>(&mut self, f: F) -> Option<V>
    where
        F: Fn(&K, &V) -> bool,
    {
        for (k, v) in self.iter.by_ref() {
            if f(&k, &v) {
                return self.table.delete_one(k);
            }
        }
        None
    }
}

pub struct Recall<'a, K, V, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K, &V) -> bool,
{
    pub(super) f: F,
    pub(super) inner: RawRecall<'a, K, V>,
}

impl<K, V, F> Iterator for Recall<'_, K, V, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
    F: Fn(&K, &V) -> bool,
{
    type Item = V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|k, v| (self.f)(k, v))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.iter.size_hint().1)
    }
}

impl<K, V, F> iter::FusedIterator for Recall<'_, K, V, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
    F: Fn(&K, &V) -> bool,
{
}

impl<K, V, F> iter::ExactSizeIterator for Recall<'_, K, V, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
    F: Fn(&K, &V) -> bool,
{
}
