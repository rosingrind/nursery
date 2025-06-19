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
pub struct SparMap<K: Unsigned, T> {
    keys: SparSet<K, KEY_MAX>,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: Box<[T]>,
}

impl<K, T> Default for SparMap<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, T> SparMap<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    pub const MAX_K: usize = 2usize.pow(size_of::<K>() as u32 * 8) - 1;

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        Self {
            keys: SparSet::new(),
            vals: unsafe { Box::new_uninit_slice(Key::MAX as usize).assume_init() },
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
    pub fn as_keys_set(&self) -> &impl SetRef<K, KEY_MAX> {
        &self.keys
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals(&self) -> &[T] {
        let len = self.len().as_();
        &self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals_mut(&mut self) -> &mut [T] {
        let len = self.len().as_();
        &mut self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one(&self, k: K) -> Option<&T> {
        self.keys.as_index_one(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one_mut(&mut self, k: K) -> Option<&mut T> {
        self.keys.as_index_one(k).map(|k| &mut self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all(&self, k: &[K]) -> impl Iterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all(&self, k: &[K]) -> impl ParallelIterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k.as_()])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut(&mut self, k: &[K]) -> impl Iterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all_mut(&mut self, k: &[K]) -> impl ParallelIterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k.as_()) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, i: K) -> bool {
        self.keys.contains(i)
    }

    pub(crate) fn filter_all_excl(&self, kv: &[(K, T)]) -> (Vec<K>, Vec<T>) {
        let mut bit = bitvec::bitbox![0; Self::MAX_K];
        let mut k = Vec::with_capacity(kv.len());
        let mut v = Vec::with_capacity(kv.len());

        for (i, x) in kv.iter() {
            if !bit[i.as_()] && !self.keys.contains(*i) {
                bit.set(i.as_(), true);
                k.push(*i);
                v.push(*x);
            }
        }

        (k, v)
    }
}

impl<'a, K, T> IntoIterator for &'a SparMap<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = (&'a K, &'a T);
    type IntoIter = impl_ref::MapIter<'a, K, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, K, T> {
        self.iter()
    }
}

impl<'a, K, T> IntoIterator for &'a mut SparMap<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = (&'a K, &'a T);
    type IntoIter = impl_ref::MapIter<'a, K, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, K, T> {
        self.iter()
    }
}
