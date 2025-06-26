mod impl_mut;
mod impl_ops;
mod impl_ref;
#[cfg(test)]
mod tests;

#[cfg(feature = "mmap")]
use std::fs::File;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
#[cfg(feature = "mmap")]
use memmap2::MmapMut;
use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::SetMut;
pub use impl_ref::SetRef;

#[cfg_attr(feature = "bitcode", derive(Decode, Encode))]
#[cfg_attr(not(feature = "mmap"), derive(Clone))]
pub struct SparSet<K: Unsigned> {
    sparse: Box<[K]>,
    len: K,
    dense: Box<[K]>,
    #[cfg(feature = "mmap")]
    mmap: MmapMut,
}

impl<K> Default for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new(SparSet::<K>::MAX_K)
    }
}

impl<K> SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub const MAX_K: usize = 2usize.pow(size_of::<K>() as u32 * 8) - 1;

    #[cfg_attr(feature = "inline-more", inline)]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        assert!(N <= Self::MAX_K);

        Self {
            sparse: unsafe { Box::new_uninit_slice(N + 1).assume_init() },
            len: K::zero(),
            dense: unsafe { Box::new_uninit_slice(N + 1).assume_init() },
            #[cfg(feature = "mmap")]
            mmap: {
                let file = File::open("LICENSE-APACHE").unwrap();
                unsafe { memmap2::MmapMut::map_mut(&file).unwrap() }
            },
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
    /// Returns dense index of key if exists
    pub(crate) fn as_index_one_uncheck(&self, k: K) -> K {
        self.sparse[k.as_()]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    /// Returns dense indexes of keys
    pub fn as_index_all<I: IntoIterator<Item = K>>(&self, k: I) -> impl Iterator<Item = K> {
        k.into_iter().filter_map(|k| self.as_index_one(k))
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    /// Returns dense indexes of keys (parallel)
    pub fn as_index_all<I: IntoParallelIterator<Item = K>>(
        &self,
        k: I,
    ) -> impl ParallelIterator<Item = K>
    where
        K: Send + Sync,
        <I as IntoParallelIterator>::Iter: IndexedParallelIterator,
    {
        k.into_par_iter().filter_map(|k| self.as_index_one(k))
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, k: K) -> bool {
        if branches::likely(k.as_() < self.sparse.len()) {
            let x = self.sparse[k.as_()];
            x < self.len && self.dense[x.as_()] == k
        } else {
            false
        }
    }

    #[inline]
    pub(crate) fn insert_one_seq_uncheck(&mut self, k: K) {
        self.sparse[k.as_()] = self.len;
        self.dense[self.len.as_()] = k;
        self.len = self.len.add(K::one());
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn insert_all_seq_uncheck<I: IntoIterator<Item = K>>(&mut self, a: I) {
        for k in a {
            self.sparse[k.as_()] = self.len;
            self.dense[self.len.as_()] = k;
            self.len = self.len.add(K::one());
        }
    }

    #[inline]
    pub(crate) fn delete_one_seq_uncheck(&mut self, k: K) {
        let s = self.sparse[k.as_()];
        self.len = self.len.sub(K::one());
        self.sparse[self.dense[self.len.as_()].as_()] = self.sparse[k.as_()];
        self.dense[s.as_()] = self.dense[self.len.as_()];
    }

    #[inline]
    pub(crate) fn delete_all_seq_uncheck<I: IntoIterator<Item = K>>(&mut self, a: I) {
        // < 25%
        for k in a {
            self.delete_one_seq_uncheck(k);
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn filter_all_dups<I: IntoIterator<Item = K>>(
        &self,
        k: I,
    ) -> impl Iterator<Item = K> + use<I, K> {
        let mut bit = bitvec::BitVec::zeros(Self::MAX_K);
        k.into_iter().filter(move |&i| {
            if branches::likely(!bit.get(i.as_()).unwrap()) {
                bit.set(i.as_(), true);
                true
            } else {
                false
            }
        })
    }
}

impl<'a, K> IntoIterator for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;
    type IntoIter = impl_ref::SetIter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K> IntoIterator for &'a mut SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;
    type IntoIter = impl_ref::SetIter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, K> IntoParallelIterator for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Sync,
{
    type Item = &'a K;
    type Iter = impl_ref::SetParIter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        <SparSet<K> as SetRef<K>>::par_iter(self)
    }
}

#[cfg(feature = "rayon")]
impl<'a, K> IntoParallelIterator for &'a mut SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Sync,
{
    type Item = &'a K;
    type Iter = impl_ref::SetParIter<'a, K>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_par_iter(self) -> Self::Iter {
        <SparSet<K> as SetRef<K>>::par_iter(self)
    }
}
