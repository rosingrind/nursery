mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use crate::set::{SetMut, SetRef};

use super::{MapRef, model::*};

pub trait MapMut<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn clear(&mut self);

    /// Retain entries specified by predicate
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K, &V) -> bool;

    /// Lazy recall operation
    ///
    /// Removes entries specified by predicate and returns
    /// an iterator over deleted's values
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, V, F>
    where
        F: Fn(&K, &V) -> bool;

    /// Insert entry or return old value if existed
    fn insert_one(&mut self, k: K, v: V) -> Option<V>;

    /// Batched insert operation
    fn insert_all<I: IntoIterator<Item = (K, V)>>(&mut self, kv: I);

    /// Delete entry and return it's value if deleted
    fn delete_one(&mut self, k: K) -> Option<V>;

    /// Batched delete operation
    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I);
}

impl<K, V> MapMut<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Copy,
{
    fn clear(&mut self) {
        self.k().clear();
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K, &V) -> bool,
    {
        let mut i = 0usize;
        while likely_stable::likely(i < self.len().as_()) {
            let (keys, vals) = self.kv();
            let k = keys.as_slice()[i];
            let v = &vals[i];
            let cond = !f(&k, v);
            i += std::hint::select_unpredictable(cond, 0, 1);
            if cond {
                self.delete_one_seq_uncheck(k);
            }
        }
    }

    fn recall<F>(&mut self, f: F) -> Recall<'_, K, V, F>
    where
        F: Fn(&K, &V) -> bool,
    {
        let vec: Vec<_> = self.iter().map(|(k, v)| (*k, *v)).collect();

        Recall {
            f,
            inner: RawRecall {
                iter: vec.into_iter(),
                table: self,
            },
        }
    }

    fn insert_one(&mut self, k: K, v: V) -> Option<V> {
        let cond = self.k().insert_one(k);

        let k = std::hint::select_unpredictable(
            cond,
            self.k().len().as_() - 1,
            self.k().as_index_one_uncheck(k).as_(),
        );
        let old = std::hint::select_unpredictable(cond, None, self.v().get(k).copied());
        self.v()[k] = v;
        old
    }

    fn insert_all<I: IntoIterator<Item = (K, V)>>(&mut self, kv: I) {
        for (k, v) in kv {
            let cond = self.k().insert_one(k);
            let k = std::hint::select_unpredictable(
                cond,
                self.k().len().as_() - 1,
                self.k().as_index_one_uncheck(k).as_(),
            );
            self.v()[k] = v;
        }
    }

    fn delete_one(&mut self, k: K) -> Option<V> {
        self.k().as_index_one(k).map(|i| {
            self.k().delete_one_seq_uncheck(k);
            let v = self.v()[i.as_()];
            let l = self.len().as_();
            self.v()[i.as_()] = self.v()[l];
            v
        })
    }

    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        for s in k {
            if let Some(i) = self.k().as_index_one(s) {
                self.k().delete_one_seq_uncheck(s);
                let l = self.len().as_();
                self.v()[i.as_()] = self.v()[l];
            }
        }
    }
}
