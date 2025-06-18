use std::iter::FusedIterator;

use super::{Key, SetRef, SparSet};

pub trait SetMut {
    fn clear(&mut self);

    /// Retain entries specified by predicate
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Key) -> bool;

    /// Lazy recall operation
    ///
    /// Removes entries specified by predicate and returns
    /// an iterator over deleted values
    fn recall<F>(&mut self, f: F) -> Recall<'_, F>
    where
        F: Fn(&Key) -> bool;

    /// Insert entry and return operation's result
    fn insert_one(&mut self, k: Key) -> bool;

    /// Batched insert operation
    ///
    /// Returns existing's owned value vec
    fn insert_all(&mut self, k: Vec<Key>);

    /// Delete entry and return operation's result
    fn delete_one(&mut self, k: Key) -> bool;

    /// Batched delete operation
    fn delete_all(&mut self, k: Vec<Key>);
}

impl SetMut for SparSet {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
        #[cfg(feature = "bitmask")]
        {
            self.mask = KeySliceMask::ZERO;
        }
        self.len = 0;
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Key) -> bool,
    {
        let mut vec = Vec::with_capacity(self.len() as usize);
        for item in self.iter() {
            if !f(item) {
                vec.push(*item);
            }
        }
        self.delete_all_seq_uncheck(&vec, |_, _| {});
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, F>
    where
        F: Fn(&Key) -> bool,
    {
        Recall {
            f,
            inner: RawRecall {
                #[allow(clippy::unnecessary_to_owned)]
                iter: self.as_slice().to_vec().into_iter(),
                table: self,
            },
        }
    }

    fn insert_one(&mut self, k: Key) -> bool {
        if self.contains(k) {
            return false;
        }

        #[cfg(feature = "bitmask")]
        {
            self.mask.set(k as usize, true);
        }
        self.sparse[k as usize] = self.len;
        self.dense[self.len as usize] = k;
        self.len += 1;

        true
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all(&mut self, k: Vec<Key>) {
        let s = self.filter_all_excl(&k);

        self.insert_all_seq_uncheck(&s);
    }

    fn delete_one(&mut self, k: Key) -> bool {
        if !self.contains(k) {
            return false;
        }

        self.delete_one_seq_uncheck(k, |_, _| {});

        true
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all(&mut self, k: Vec<Key>) {
        let s = self.filter_all_incl(&k);

        self.delete_all_seq_uncheck(&s, |_, _| {});
    }
}

pub(crate) struct RawRecall<'a> {
    pub iter: std::vec::IntoIter<Key>,
    pub table: &'a mut SparSet,
}

impl RawRecall<'_> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub(crate) fn next<F>(&mut self, f: F) -> Option<Key>
    where
        F: Fn(&Key) -> bool,
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

pub struct Recall<'a, F>
where
    F: Fn(&Key) -> bool,
{
    f: F,
    inner: RawRecall<'a>,
}

impl<F> Iterator for Recall<'_, F>
where
    F: Fn(&Key) -> bool,
{
    type Item = Key;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|k| (self.f)(k))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.inner.iter.size_hint().1)
    }
}

impl<F> FusedIterator for Recall<'_, F> where F: Fn(&Key) -> bool {}

impl<F> ExactSizeIterator for Recall<'_, F> where F: Fn(&Key) -> bool {}
