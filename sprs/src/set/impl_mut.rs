mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use super::{SetRef, SparSet};

pub trait SetMut<K, const N: usize>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn clear(&mut self);

    /// Retain entries specified by predicate
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K) -> bool;

    /// Lazy recall operation
    ///
    /// Removes entries specified by predicate and returns
    /// an iterator over deleted values
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, N, F>
    where
        F: Fn(&K) -> bool;

    /// Insert entry and return operation's result
    fn insert_one(&mut self, k: K) -> bool;

    /// Batched insert operation
    ///
    /// Returns existing's owned value vec
    fn insert_all(&mut self, k: Vec<K>);

    /// Delete entry and return operation's result
    fn delete_one(&mut self, k: K) -> bool;

    /// Batched delete operation
    fn delete_all(&mut self, k: Vec<K>);
}

impl<K, const N: usize> SetMut<K, N> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
        #[cfg(feature = "bitmask")]
        {
            self.mask = KeySliceMask::ZERO;
        }
        self.len = K::zero();
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K) -> bool,
    {
        let mut vec = Vec::with_capacity(self.len().as_());
        for item in self.iter() {
            if !f(item) {
                vec.push(*item);
            }
        }
        self.delete_all_seq_uncheck(&vec, |_, _| {});
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, N, F>
    where
        F: Fn(&K) -> bool,
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

    fn insert_one(&mut self, k: K) -> bool {
        assert!(k.as_() <= N);

        if self.contains(k) {
            return false;
        }

        #[cfg(feature = "bitmask")]
        {
            self.mask.set(k.as_(), true);
        }
        self.sparse[k.as_()] = self.len;
        self.dense[self.len.as_()] = k;
        self.len = self.len.add(K::one());
        // self.len += 1;

        true
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all(&mut self, k: Vec<K>) {
        let s = self.filter_all_excl(&k);

        self.insert_all_seq_uncheck(&s);
    }

    fn delete_one(&mut self, k: K) -> bool {
        if !self.contains(k) {
            return false;
        }

        self.delete_one_seq_uncheck(k, |_, _| {});

        true
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all(&mut self, k: Vec<K>) {
        let s = self.filter_all_incl(&k);

        self.delete_all_seq_uncheck(&s, |_, _| {});
    }
}
