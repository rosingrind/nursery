mod impl_mut;
mod impl_ops;
mod impl_ref;
mod model;
#[cfg(test)]
mod tests;

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::SetMut;
pub use impl_ref::SetRef;

pub use model::*;

impl<K: Unsigned> Default for SparSet<K> {
    fn default() -> Self {
        Self::new(SparSet::<K>::MAX_K)
    }
}

/// Unified models' creation facade
impl<K: Unsigned> SparSet<K> {
    pub const MAX_K: usize = 2usize.pow(size_of::<K>() as u32 * 8) - 1;

    #[cfg(feature = "volatile")]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        assert!(N <= Self::MAX_K);

        Self::from_raw(
            K::zero(),
            unsafe { Box::new_uninit_slice(N + 1).assume_init() },
            unsafe { Box::new_uninit_slice(N + 1).assume_init() },
        )
    }

    #[cfg(feature = "memmap2")]
    #[allow(non_snake_case)]
    pub fn new(N: usize) -> Self {
        let file = tempfile::tempfile().unwrap();
        file.set_len(Self::file_size(N)).unwrap();

        Self::from_buf(N, file)
    }
}

impl<K> SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub fn len(&self) -> K {
        *self.l()
    }

    pub fn is_empty(&self) -> bool {
        *self.l() == K::zero()
    }

    /// Returns dense index of key if exists
    pub fn as_index_one(&self, k: K) -> Option<K> {
        if self.contains(k) {
            Some(self.s()[k.as_()])
        } else {
            None
        }
    }

    /// Returns dense index of key if exists
    pub(crate) fn as_index_one_uncheck(&self, k: K) -> K {
        self.s()[k.as_()]
    }

    #[cfg(not(feature = "rayon"))]
    /// Returns dense indexes of keys
    pub fn as_index_all<I: IntoIterator<Item = K>>(&self, k: I) -> impl Iterator<Item = K> {
        k.into_iter().filter_map(|k| self.as_index_one(k))
    }

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

    #[inline]
    pub(crate) fn fittable(&self, k: K) -> bool {
        k.as_() < self.s().len()
    }

    pub fn contains(&self, k: K) -> bool {
        self.fittable(k)
            && (self.s()[k.as_()] < *self.l()) & (self.d()[self.s()[k.as_()].as_()] == k)
    }

    #[inline]
    pub(crate) fn insert_one_seq_uncheck(&mut self, k: K) {
        self.s()[k.as_()] = *self.l();
        let l = self.l().as_(); // TODO: get rid of temporary variable
        self.d()[l] = k;
        *self.l() = self.l().add(K::one());
    }

    #[allow(dead_code)]
    #[inline]
    pub(crate) fn insert_all_seq_uncheck<I: IntoIterator<Item = K>>(&mut self, a: I) {
        for k in a {
            self.insert_one_seq_uncheck(k);
        }
    }

    #[inline]
    pub(crate) fn delete_one_seq_uncheck(&mut self, k: K) {
        let s = self.s()[k.as_()];
        *self.l() = self.l().sub(K::one());
        let l = self.l().as_(); // TODO: get rid of temporary variable
        let d = self.d()[l].as_(); // TODO: get rid of temporary variable
        self.s()[d] = self.s()[k.as_()];
        self.d()[s.as_()] = self.d()[l];
    }

    #[allow(dead_code)]
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
            if likely_stable::likely(!bit.get(i.as_()).unwrap()) {
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

    fn into_par_iter(self) -> Self::Iter {
        <SparSet<K> as SetRef<K>>::par_iter(self)
    }
}
