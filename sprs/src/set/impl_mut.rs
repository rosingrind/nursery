mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use super::{SetRef, SparSet};

pub trait SetMut<K>
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
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, F>
    where
        F: Fn(&K) -> bool;

    /// Insert entry and return operation's result
    fn insert_one(&mut self, k: K) -> bool;

    /// Batched insert operation
    ///
    /// Returns existing's owned value vec
    fn insert_all<I: IntoIterator<Item = K>>(&mut self, k: I);

    /// Delete entry and return operation's result
    fn delete_one(&mut self, k: K) -> bool;

    /// Batched delete operation
    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I);
}

impl<K> SetMut<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
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
        self.delete_all_seq_uncheck(vec);
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, F>
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
        assert!(k.as_() <= self.sparse.len());

        if branches::likely(!self.contains(k)) {
            self.insert_one_seq_uncheck(k);
            true
        } else {
            false
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        for s in k {
            let _ = self.insert_one(s);
        }
    }

    fn delete_one(&mut self, k: K) -> bool {
        if branches::likely(self.contains(k)) {
            self.delete_one_seq_uncheck(k);
            true
        } else {
            false
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        for s in k {
            let _ = self.delete_one(s);
        }
    }
}
