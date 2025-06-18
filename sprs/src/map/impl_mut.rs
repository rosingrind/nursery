use std::iter::FusedIterator;

use crate::set::SetMut;

use super::{Key, MapRef, SparMap};

pub trait MapMut<T> {
    fn clear(&mut self);

    /// Retain entries specified by predicate
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Key, &T) -> bool;

    /// Lazy recall operation
    ///
    /// Removes entries specified by predicate and returns
    /// an iterator over deleted's values
    fn recall<F>(&mut self, f: F) -> Recall<'_, T, F>
    where
        F: Fn(&Key, &T) -> bool;

    /// Insert entry or return old value if existed
    fn insert_one(&mut self, k: Key, v: T) -> Option<T>;

    /// Batched insert operation
    fn insert_all(&mut self, kv: Vec<(Key, T)>);

    /// Delete entry and return it's value if deleted
    fn delete_one(&mut self, k: Key) -> Option<T>;

    /// Batched delete operation
    fn delete_all(&mut self, k: &[Key]);
}

impl<T: Send + Sync + Copy> MapMut<T> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
        self.keys.clear();
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&Key, &T) -> bool,
    {
        let mut vec = Vec::with_capacity(self.len() as usize);
        for (k, v) in self.iter() {
            if !f(k, v) {
                vec.push(*k);
            }
        }
        self.delete_all(&vec);
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, T, F>
    where
        F: Fn(&Key, &T) -> bool,
    {
        Recall {
            f,
            inner: RawRecall {
                iter: self
                    .iter()
                    .map(|(k, v)| (*k, *v))
                    .collect::<Vec<_>>()
                    .into_iter(),
                table: self,
            },
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_one(&mut self, k: Key, v: T) -> Option<T> {
        if !self.keys.insert_one(k) {
            let k = self.keys.as_index_one(k).unwrap();
            let old = self.vals[k as usize];
            self.vals[k as usize] = v;
            return Some(old);
        }
        self.vals[self.keys.len() as usize - 1] = v;
        None
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all(&mut self, kv: Vec<(Key, T)>) {
        let (k, v) = self.filter_all_excl(&kv);

        let len = self.len() as usize;
        self.keys.insert_all_seq_uncheck(&k);
        self.vals[len..(len + v.len())].copy_from_slice(v.as_slice());
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_one(&mut self, k: Key) -> Option<T> {
        self.contains(k).then(|| {
            self.keys
                .delete_one_seq_uncheck(k, |k, l| self.vals.swap(k as usize, l as usize));
            self.vals[self.len() as usize]
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all(&mut self, k: &[Key]) {
        self.keys
            .delete_all_seq_uncheck(&self.keys.filter_all_incl(k), |k, l| {
                self.vals.swap(k as usize, l as usize)
            });
    }
}

pub(crate) struct RawRecall<'a, T> {
    pub iter: std::vec::IntoIter<(Key, T)>,
    pub table: &'a mut SparMap<T>,
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
    f: F,
    inner: RawRecall<'a, T>,
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

impl<T, F> FusedIterator for Recall<'_, T, F>
where
    T: Send + Sync + Copy,
    F: Fn(&Key, &T) -> bool,
{
}

impl<T, F> ExactSizeIterator for Recall<'_, T, F>
where
    T: Send + Sync + Copy,
    F: Fn(&Key, &T) -> bool,
{
}
