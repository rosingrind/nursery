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

pub use impl_mut::SetMut;
pub use impl_ref::SetRef;

#[cfg_attr(feature = "bitcode", derive(Decode, Encode))]
#[derive(Clone)]
pub struct SparSet<K: Unsigned, const N: usize> {
    sparse: Box<[K]>,
    len: K,
    dense: Box<[K]>,
    #[cfg(feature = "bitmask")]
    /// bit mask representing all set element, requires `feature = "bitmask"`
    mask: BitBox,
}

impl<K, const N: usize> Default for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new()
    }
}

impl<K, const N: usize> SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub const MAX_K: usize = 2usize.pow(size_of::<K>() as u32 * 8) - 1;

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        assert!(N <= Self::MAX_K);

        Self {
            sparse: unsafe { Box::new_uninit_slice(N + 1).assume_init() },
            len: K::zero(),
            dense: unsafe { Box::new_uninit_slice(N + 1).assume_init() },
            #[cfg(feature = "bitmask")]
            mask: bitvec::bitbox![0; Self::MAX_K],
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> K {
        self.len
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {
        self.len == K::zero()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    /// Returns dense index of key if exists
    pub fn as_index_one(&self, k: K) -> Option<K> {
        if self.contains(k) {
            Some(self.sparse[k.as_()])
        } else {
            None
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    /// Returns dense indexes of keys
    pub fn as_index_all(&self, k: &[K]) -> impl Iterator<Item = K> {
        k.iter()
            .filter_map(|k| self.contains(*k).then_some(self.sparse[k.as_()]))
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    /// Returns dense indexes of keys (parallel)
    pub fn as_index_all(&self, k: &[K]) -> impl ParallelIterator<Item = K> {
        k.par_iter()
            .filter_map(|k| self.contains(*k).then_some(self.sparse[k.as_()]))
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, k: K) -> bool {
        if k.as_() > N {
            return false;
        }
        let x = self.sparse[k.as_()];
        #[cfg(not(feature = "bitmask"))]
        {
            x < self.len && self.dense[x.as_()] == k
            // dbg!(dbg!(x) < dbg!(self.len)) && dbg!(dbg!(self.dense[x.as_()]) == dbg!(k))
        }
        #[cfg(feature = "bitmask")]
        {
            x < self.len && self.dense[k.as_()] == k && self.mask[k as usize]
        }
    }

    #[inline]
    pub(crate) fn insert_all_seq_uncheck(&mut self, a: &[K]) {
        self.dense[self.len.as_()..(self.len.as_() + a.len())].copy_from_slice(a);

        for k in a.iter() {
            #[cfg(feature = "bitmask")]
            {
                self.mask.set(k.as_(), true);
            }
            self.sparse[k.as_()] = self.len;
            self.len = self.len.add(K::one());
            // self.len += 1;
        }
    }

    // TODO: test if `#[inline]` elides calling `f()`
    #[inline]
    pub(crate) fn delete_one_seq_uncheck<F: FnMut(K, K)>(&mut self, k: K, mut f: F) {
        #[cfg(feature = "bitmask")]
        {
            self.mask.set(k.as_(), false);
        }
        let s = self.sparse[k.as_()];
        self.len = self.len.sub(K::one());
        // self.len -= 1;
        f(s, self.len);
        self.sparse.swap(k.as_(), self.dense[self.len.as_()].as_());
        self.dense.swap(s.as_(), self.len.as_());
    }

    // TODO: test if `#[inline]` elides calling `f()`
    #[inline]
    #[cfg(not(feature = "rayon"))]
    /// "swap + pop" (seq) deletion of multiple entries
    pub(crate) fn delete_all_seq_uncheck<F: FnMut(K, K)>(&mut self, a: &[K], mut f: F) {
        // < 25%
        for &k in a {
            #[cfg(feature = "bitmask")]
            {
                self.mask.set(k.as_(), false);
            }
            let s = self.sparse[k.as_()];
            self.len = self.len.sub(K::one());
            // self.len -= 1;
            f(s, self.len);
            self.sparse.swap(k.as_(), self.dense[self.len.as_()].as_());
            self.dense.swap(s.as_(), self.len.as_());
        }
    }

    #[cfg(feature = "rayon")]
    /// "swap + pop" (seq) deletion of multiple entries
    pub(crate) fn delete_all_seq_uncheck(&mut self, a: &mut [Key]) {
        #[cfg(feature = "bitmask")]
        {
            for k in a.iter() {
                self.mask.set(*k as usize, true);
            }
        }
        // < 25%
        a.par_iter_mut().enumerate().for_each(|(i, k)| {
            let s = self.sparse[*k as usize];

            let ptr_s = self.sparse.as_ptr();
            let raw_s = ptr_s as *mut Key;
            unsafe {
                raw_s
                    .add(*k as usize)
                    .swap(raw_s.add(self.dense[self.len as usize - i - 1] as usize))
            };

            let ptr_d = self.dense.as_ptr();
            let raw_d = ptr_d as *mut Key;
            unsafe {
                raw_d
                    .add(s as usize)
                    .swap(raw_d.add(self.len as usize - i - 1))
            };

            *k = s;
        });

        self.len -= a.len() as Key;
    }

    pub(crate) fn filter_all_incl(&self, k: &[K]) -> Vec<K> {
        let mut bit = bitvec::bitbox![0; Self::MAX_K];
        let mut res = Vec::with_capacity(k.len());

        for i in k.iter() {
            if !bit[i.as_()] && self.contains(*i) {
                bit.set(i.as_(), true);
                res.push(*i);
            }
        }

        res
    }

    pub(crate) fn filter_all_excl(&self, k: &[K]) -> Vec<K> {
        let mut bit = bitvec::bitbox![0; Self::MAX_K];
        let mut res = Vec::with_capacity(k.len());

        for i in k.iter() {
            if !bit[i.as_()] && !self.contains(*i) {
                bit.set(i.as_(), true);
                res.push(*i);
            }
        }

        res
    }
}

impl<'a, K, const N: usize> IntoIterator for &'a SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;
    type IntoIter = std::slice::Iter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> std::slice::Iter<'a, K> {
        self.iter()
    }
}

impl<'a, K, const N: usize> IntoIterator for &'a mut SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;
    type IntoIter = std::slice::Iter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> std::slice::Iter<'a, K> {
        self.iter()
    }
}
