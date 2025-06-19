mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use crate::set::SetMut;

use super::{MapRef, SparMap};

pub trait MapMut<K, T, const N: usize>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn clear(&mut self);

    /// Retain entries specified by predicate
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K, &T) -> bool;

    /// Lazy recall operation
    ///
    /// Removes entries specified by predicate and returns
    /// an iterator over deleted's values
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, T, N, F>
    where
        F: Fn(&K, &T) -> bool;

    /// Insert entry or return old value if existed
    fn insert_one(&mut self, k: K, v: T) -> Option<T>;

    /// Batched insert operation
    fn insert_all(&mut self, kv: Vec<(K, T)>);

    /// Delete entry and return it's value if deleted
    fn delete_one(&mut self, k: K) -> Option<T>;

    /// Batched delete operation
    fn delete_all(&mut self, k: &[K]);
}

impl<K, T, const N: usize> MapMut<K, T, N> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
        self.keys.clear();
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K, &T) -> bool,
    {
        let mut vec = Vec::with_capacity(self.len().as_());
        for (k, v) in self.iter() {
            if !f(k, v) {
                vec.push(*k);
            }
        }
        self.delete_all(&vec);
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, T, N, F>
    where
        F: Fn(&K, &T) -> bool,
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
    fn insert_one(&mut self, k: K, v: T) -> Option<T> {
        if !self.keys.insert_one(k) {
            let k = self.keys.as_index_one(k).unwrap();
            let old = self.vals[k.as_()];
            self.vals[k.as_()] = v;
            return Some(old);
        }
        self.vals[self.keys.len().as_() - 1] = v;
        None
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all(&mut self, kv: Vec<(K, T)>) {
        let (k, v) = self.filter_all_excl(&kv);

        let len = self.len().as_();
        self.keys.insert_all_seq_uncheck(&k);
        self.vals[len..(len + v.len())].copy_from_slice(v.as_slice());
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_one(&mut self, k: K) -> Option<T> {
        self.contains(k).then(|| {
            self.keys
                .delete_one_seq_uncheck(k, |k, l| self.vals.swap(k.as_(), l.as_()));
            self.vals[self.len().as_()]
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all(&mut self, k: &[K]) {
        self.keys
            .delete_all_seq_uncheck(&self.keys.filter_all_incl(k), |k, l| {
                self.vals.swap(k.as_(), l.as_())
            });
    }
}
