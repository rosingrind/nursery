mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use super::{SetRef, model::*};

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
    fn clear(&mut self) {
        self.l().set_zero();
    }

    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K) -> bool,
    {
        let mut i = 0usize;
        while likely_stable::likely(i < self.l().as_()) {
            let k = &self.as_slice()[i];
            let cond = !f(k);
            i += std::hint::select_unpredictable(cond, 0, 1);
            if cond {
                self.delete_one_seq_uncheck(*k);
            }
        }
    }

    fn recall<F>(&mut self, f: F) -> Recall<'_, K, F>
    where
        F: Fn(&K) -> bool,
    {
        Recall {
            f,
            inner: RawRecall {
                pos: 0usize,
                table: self,
            },
        }
    }

    fn insert_one(&mut self, k: K) -> bool {
        if likely_stable::unlikely(!self.fittable(k)) {
            panic!("k is larger than sparse limit");
        }

        let cond = !self.contains(k);
        if cond {
            self.insert_one_seq_uncheck(k);
        }
        cond
    }

    fn insert_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        for s in k {
            let _ = self.insert_one(s);
        }
    }

    fn delete_one(&mut self, k: K) -> bool {
        let cond = self.contains(k);
        if cond {
            self.delete_one_seq_uncheck(k);
        }
        cond
    }

    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        for s in k {
            let _ = self.delete_one(s);
        }
    }
}
