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
    KEY_MAX, Key,
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
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new(SparMap::<K, V>::MAX_K)
    }
}

impl<K, V> SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    pub const MAX_K: usize = 2usize.pow(size_of::<K>() as u32 * 8) - 1;

    #[cfg_attr(feature = "inline-more", inline)]
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
    pub fn query_all(&self, k: &[K]) -> impl Iterator<Item = &V> {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all(&self, k: &[K]) -> impl ParallelIterator<Item = &V>
    where
        K: Send + Sync,
    {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut(&mut self, k: &[K]) -> impl Iterator<Item = &mut V> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut V;
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all_mut(&mut self, k: &[K]) -> impl ParallelIterator<Item = &mut V>
    where
        K: Send + Sync,
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

    pub(crate) fn filter_all_excl(&self, kv: Vec<(K, V)>) -> (Vec<K>, Vec<V>) {
        let mut bit = bitvec::bitbox![0; Self::MAX_K];
        let mut k = Vec::with_capacity(kv.len());
        let mut v = Vec::with_capacity(kv.len());

        for (i, x) in kv.into_iter() {
            if !bit[i.as_()] && !self.keys.contains(i) {
                bit.set(i.as_(), true);
                k.push(i);
                v.push(x);
            }
        }

        (k, v)
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
