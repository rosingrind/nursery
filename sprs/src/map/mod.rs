mod impl_mut;
mod impl_ops;
mod impl_ref;
#[cfg(test)]
mod tests;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::MapMut;
pub use impl_ref::MapRef;

use crate::{
    Key,
    set::{SetRef, SparSet},
};

pub type MapSlice<T> = [T; Key::MAX as usize];
type MapSliceMask = bitvec::BitArr!(for Key::MAX as usize, in Key);

// TOOD: sorted criteria; for example you can store `Pos { x, y }`
// as `x` and `y`, and sort each other - this allows to query
// by position (find all by `x` in range, by `y` in range, intersect results)
#[cfg_attr(feature = "bitcode", derive(Decode, Encode))]
#[derive(Clone)]
pub struct SparMap<T> {
    keys: SparSet<Key>,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: Box<[T]>,
}

impl<T: Send + Sync + Copy> Default for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync + Copy> SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        Self {
            keys: SparSet::new(),
            vals: unsafe { Box::new_uninit_slice(Key::MAX as usize).assume_init() },
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> Key {
        self.keys.len()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys(&self) -> &[Key] {
        self.keys.as_slice()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys_set(&self) -> &impl SetRef<Key> {
        &self.keys
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals(&self) -> &[T] {
        let len = self.len() as usize;
        &self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals_mut(&mut self) -> &mut [T] {
        let len = self.len() as usize;
        &mut self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one(&self, k: Key) -> Option<&T> {
        self.keys.as_index_one(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one_mut(&mut self, k: Key) -> Option<&mut T> {
        self.keys
            .as_index_one(k)
            .map(|k| &mut self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all(&self, k: &[Key]) -> impl Iterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all(&self, k: &[Key]) -> impl ParallelIterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut(&mut self, k: &[Key]) -> impl Iterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k as usize) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all_mut(&mut self, k: &[Key]) -> impl ParallelIterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k as usize) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, i: Key) -> bool {
        self.keys.contains(i)
    }

    pub(crate) fn filter_all_excl(&self, kv: &[(Key, T)]) -> (Vec<Key>, Vec<T>) {
        let mut bit = MapSliceMask::ZERO;
        let mut k = Vec::with_capacity(kv.len());
        let mut v = Vec::with_capacity(kv.len());

        for (i, x) in kv.iter() {
            if !bit[*i as usize] && !self.keys.contains(*i) {
                bit.set(*i as usize, true);
                k.push(*i);
                v.push(*x);
            }
        }

        (k, v)
    }
}

impl<'a, T> IntoIterator for &'a SparMap<T> {
    type Item = (&'a Key, &'a T);
    type IntoIter = impl_ref::MapIter<'a, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SparMap<T> {
    type Item = (&'a Key, &'a T);
    type IntoIter = impl_ref::MapIter<'a, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, T> {
        self.iter()
    }
}
