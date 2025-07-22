mod impl_mut;
mod impl_ops;
mod impl_ref;
mod model;
#[cfg(test)]
mod tests;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::MapMut;
pub use impl_ref::MapRef;

pub use model::*;

use crate::set::{SetRef, SparSet};

impl<K, V> Default for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Copy,
{
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

    #[cfg(feature = "volatile")]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        Self::from_raw(SparSet::new(N), unsafe {
            Box::new_uninit_slice(N + 1).assume_init()
        })
    }

    #[cfg(feature = "memmap2")]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        let file = tempfile::tempfile().unwrap();
        file.set_len(Self::file_size(N)).unwrap();

        Self::from_buf(N, file)
    }

    pub fn len(&self) -> K {
        self.k().len()
    }

    pub fn is_empty(&self) -> bool {
        self.k().is_empty()
    }

    pub fn as_keys(&self) -> &[K] {
        self.k().as_slice()
    }

    pub fn as_keys_set(&self) -> &impl SetRef<K> {
        self.k()
    }

    pub fn as_vals(&self) -> &[V] {
        let len = self.len().as_();
        &self.v()[..len]
    }

    pub fn as_vals_mut(&mut self) -> &mut [V] {
        let len = self.len().as_();
        &mut self.v()[..len]
    }

    pub fn query_one(&self, k: K) -> Option<&V> {
        self.k().as_index_one(k).map(|k| &self.v()[k.as_()])
    }

    pub fn query_one_mut(&mut self, k: K) -> Option<&mut V> {
        self.k().as_index_one(k).map(|k| &mut self.v()[k.as_()])
    }

    #[cfg(not(feature = "rayon"))]
    pub fn query_all<I: IntoIterator<Item = K>>(&self, k: I) -> impl Iterator<Item = &V> {
        self.k().as_index_all(k).map(|k| &self.v()[k.as_()])
    }

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
        self.k().as_index_all(k).map(|k| &self.v()[k.as_()])
    }

    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut<I: IntoIterator<Item = K>>(
        &mut self,
        k: I,
    ) -> impl Iterator<Item = &mut V> {
        let (keys, vals) = self.kv();
        keys.as_index_all(k).map(|k| {
            let raw = vals.as_ptr().cast_mut();
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

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
        let (keys, vals) = self.kv();
        keys.as_index_all(k).map(|k| {
            let raw = vals.as_ptr().cast_mut();
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    pub fn contains(&self, i: K) -> bool {
        self.k().contains(i)
    }

    #[inline]
    pub(crate) fn delete_one_seq_uncheck(&mut self, k: K) {
        let i = self.k().as_index_one_uncheck(k);
        self.k().delete_one_seq_uncheck(k);
        let l = self.len().as_(); // TODO: get rid of temporary variable
        self.v()[i.as_()] = self.v()[l];
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn delete_all_seq_uncheck<I: IntoIterator<Item = K>>(&mut self, a: I) {
        for s in a {
            self.delete_one_seq_uncheck(s);
        }
    }

    #[allow(dead_code)]
    pub(crate) fn filter_all_dups<I: IntoIterator<Item = (K, V)>>(
        &self,
        kv: I,
    ) -> impl Iterator<Item = (K, V)> + use<I, K, V> {
        let mut bit = bitvec::BitVec::zeros(Self::MAX_K);
        kv.into_iter().filter(move |&(i, _)| {
            if likely_stable::likely(!bit.get(i.as_()).unwrap()) {
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

    fn into_par_iter(self) -> Self::Iter {
        <SparMap<K, V> as MapRef<K, V>>::par_iter(self)
    }
}
