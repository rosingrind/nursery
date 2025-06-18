use std::iter;

use crate::{
    Key,
    map::{MapMut, SparMap},
};

pub(super) struct RawRecall<'a, T> {
    pub(super) iter: std::vec::IntoIter<(Key, T)>,
    pub(super) table: &'a mut SparMap<T>,
}

impl<T: Send + Sync + Copy> RawRecall<'_, T> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn next<F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&Key, &T) -> bool,
    {
        for (k, v) in self.iter.by_ref() {
            if f(&k, &v) {
                return self.table.delete_one(k);
            }
        }
        None
    }
}

pub struct Recall<'a, T, F>
where
    F: Fn(&Key, &T) -> bool,
{
    pub(super) f: F,
    pub(super) inner: RawRecall<'a, T>,
}

impl<T, F> Iterator for Recall<'_, T, F>
where
    T: Send + Sync + Copy,
    F: Fn(&Key, &T) -> bool,
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

impl<T, F> iter::FusedIterator for Recall<'_, T, F>
where
    T: Send + Sync + Copy,
    F: Fn(&Key, &T) -> bool,
{
}

impl<T, F> iter::ExactSizeIterator for Recall<'_, T, F>
where
    T: Send + Sync + Copy,
    F: Fn(&Key, &T) -> bool,
{
}
