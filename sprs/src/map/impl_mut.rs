mod recall;

use num_traits::{AsPrimitive, Unsigned};

use recall::*;

use crate::set::SetMut;

use super::{MapRef, SparMap};

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
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clear(&mut self) {
        self.keys.clear();
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn retain<F>(&mut self, f: F)
    where
        F: Fn(&K, &V) -> bool,
    {
        let mut vec = Vec::with_capacity(self.len().as_());
        for (k, v) in self.iter() {
            if !f(k, v) {
                vec.push(*k);
            }
        }
        self.delete_all_seq_uncheck(vec);
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn recall<F>(&mut self, f: F) -> Recall<'_, K, V, F>
    where
        F: Fn(&K, &V) -> bool,
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
    fn insert_one(&mut self, k: K, v: V) -> Option<V> {
        if branches::likely(!self.keys.insert_one(k)) {
            let k = self.keys.as_index_one(k).unwrap();
            let old = self.vals[k.as_()];
            self.vals[k.as_()] = v;
            Some(old)
        } else {
            self.vals[self.keys.len().as_() - 1] = v;
            None
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn insert_all<I: IntoIterator<Item = (K, V)>>(&mut self, kv: I) {
        // prefetch first element
        let mut iter = kv
            .into_iter()
            .inspect(|x| unsafe { branches::prefetch_read_data(std::ptr::from_ref(x).cast(), 1) })
            .peekable();
        std::hint::black_box(iter.peek());

        for (k, v) in iter {
            unsafe {
                branches::prefetch_write_data(
                    self.vals.as_mut_ptr().add(self.keys.len().as_()).cast(),
                    0,
                )
            };
            if branches::likely(self.keys.insert_one(k)) {
                self.vals[self.keys.len().as_() - 1] = v;
            } else {
                let k = self.keys.as_index_one(k).unwrap();
                self.vals[k.as_()] = v;
            }
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_one(&mut self, k: K) -> Option<V> {
        if let Some(i) = self.keys.as_index_one(k) {
            self.keys.delete_one_seq_uncheck(k);
            self.vals.swap(i.as_(), self.len().as_());
            Some(self.vals[self.len().as_()])
        } else {
            None
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn delete_all<I: IntoIterator<Item = K>>(&mut self, k: I) {
        // prefetch first element
        let mut iter = k
            .into_iter()
            .inspect(|x| unsafe { branches::prefetch_read_data(std::ptr::from_ref(x).cast(), 1) })
            .peekable();
        std::hint::black_box(iter.peek());

        for s in iter {
            unsafe {
                branches::prefetch_write_data(
                    self.vals
                        .as_mut_ptr()
                        .add(self.keys.len().as_().saturating_sub(1))
                        .cast(),
                    0,
                )
            };
            if let Some(i) = self.keys.as_index_one(s) {
                self.keys.delete_one_seq_uncheck(s);
                self.vals.swap(i.as_(), self.len().as_());
            }
        }
    }
}
