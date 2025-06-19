use std::iter;

use num_traits::{AsPrimitive, Unsigned};

use crate::map::{MapMut, SparMap};

pub(super) struct RawRecall<'a, K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: std::vec::IntoIter<(K, T)>,
    pub(super) table: &'a mut SparMap<K, T>,
}

impl<K, T> RawRecall<'_, K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn next<F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&K, &T) -> bool,
    {
        for (k, v) in self.iter.by_ref() {
            if f(&k, &v) {
                return self.table.delete_one(k);
            }
        }
        None
    }
}

pub struct Recall<'a, K, T, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    F: Fn(&K, &T) -> bool,
{
    pub(super) f: F,
    pub(super) inner: RawRecall<'a, K, T>,
}

impl<K, T, F> Iterator for Recall<'_, K, T, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
    F: Fn(&K, &T) -> bool,
{
    type Item = T;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|k, v| (self.f)(k, v))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.iter.size_hint().1)
    }
}

impl<K, T, F> iter::FusedIterator for Recall<'_, K, T, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
    F: Fn(&K, &T) -> bool,
{
}

impl<K, T, F> iter::ExactSizeIterator for Recall<'_, K, T, F>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
    F: Fn(&K, &T) -> bool,
{
}
