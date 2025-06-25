mod impl_mut;
mod impl_ops;
mod impl_ref;
#[cfg(test)]
mod tests;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::MapMut;
pub use impl_ref::MapRef;

use crate::{
    Key,
    set::{SetRef, SparSet},
};

// TOOD: sorted criteria; for example you can store `Pos { x, y }`
// as `x` and `y`, and sort each other - this allows to query
// by position (find all by `x` in range, by `y` in range, intersect results)
#[cfg_attr(feature = "bitcode", derive(Decode, Encode))]
#[derive(Clone)]
pub struct SparMap<K: Unsigned, V> {
    keys: SparSet<K>,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: Box<[V]>,
}

impl<K, V> Default for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new(SparMap::<K, V>::MAX_K)
    }
}

impl<K, V> SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Copy,
{
    pub const MAX_K: usize = SparSet::<K>::MAX_K;

    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        Self {
            keys: SparSet::new(N),
            vals: unsafe { Box::new_uninit_slice(N + 1).assume_init() },
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> K {
        self.keys.len()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys(&self) -> &[K] {
        self.keys.as_slice()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys_set(&self) -> &impl SetRef<K> {
        &self.keys
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals(&self) -> &[V] {
        let len = self.len().as_();
        &self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals_mut(&mut self) -> &mut [V] {
        let len = self.len().as_();
        &mut self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one(&self, k: K) -> Option<&V> {
        self.keys.as_index_one(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one_mut(&mut self, k: K) -> Option<&mut V> {
        self.keys.as_index_one(k).map(|k| &mut self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all<I: IntoIterator<Item = K>>(&self, k: I) -> impl Iterator<Item = &V> {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all<I: IntoParallelIterator<Item = K>>(
        &self,
        k: I,
    ) -> impl ParallelIterator<Item = &V>
    where
        K: Send + Sync,
        V: Send + Sync,
        <I as IntoParallelIterator>::Iter: IndexedParallelIterator,
    {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut<I: IntoIterator<Item = K>>(
        &mut self,
        k: I,
    ) -> impl Iterator<Item = &mut V> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut V;
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all_mut<I: IntoParallelIterator<Item = K>>(
        &mut self,
        k: I,
    ) -> impl ParallelIterator<Item = &mut V>
    where
        K: Send + Sync,
        V: Send + Sync,
        <I as IntoParallelIterator>::Iter: IndexedParallelIterator,
    {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut V;
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, i: K) -> bool {
        self.keys.contains(i)
    }

    #[inline]
    pub(crate) fn delete_all_seq_uncheck<I: IntoIterator<Item = K>>(&mut self, a: I) {
        for s in a {
            let i = self.keys.as_index_one_uncheck(s);
            self.keys.delete_one_seq_uncheck(s);
            self.vals[i.as_()] = self.vals[self.len().as_()];
        }
    }

    #[allow(dead_code)]
    pub(crate) fn filter_all_dups<I: IntoIterator<Item = (K, V)>>(
        &self,
        kv: I,
    ) -> impl Iterator<Item = (K, V)> + use<I, K, V> {
        let mut bit = bitvec::BitVec::zeros(Self::MAX_K);
        kv.into_iter().filter(move |&(i, _)| {
            if branches::likely(!bit.get(i.as_()).unwrap()) {
                bit.set(i.as_(), true);
                true
            } else {
                false
            }
        })
    }
}

impl<'a, K, V> IntoIterator for &'a SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = (&'a K, &'a V);
    type IntoIter = impl_ref::MapIter<'a, K, V>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = (&'a K, &'a V);
    type IntoIter = impl_ref::MapIter<'a, K, V>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, K, V> {
        self.iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, K, V> IntoParallelIterator for &'a SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Sync,
    V: Sync,
{
    type Item = (&'a K, &'a V);
    type Iter = impl_ref::MapParIter<'a, K, V>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        <SparMap<K, V> as MapRef<K, V>>::par_iter(self)
    }
}

#[cfg(feature = "rayon")]
impl<'a, K, V> IntoParallelIterator for &'a mut SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Sync,
    V: Sync,
{
    type Item = (&'a K, &'a V);
    type Iter = impl_ref::MapParIter<'a, K, V>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        <SparMap<K, V> as MapRef<K, V>>::par_iter(self)
    }
}
