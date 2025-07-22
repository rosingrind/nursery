use std::fs::File;

use num_traits::{AsPrimitive, Unsigned};

use crate::{ext::*, set::SparSet};

use super::{ModelMutAccess, ModelRefAccess};

pub struct SparMap<K: Unsigned, V> {
    keys: SparSet<K>,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: BufMut<V>,
}

impl<K: Unsigned, V> SparMap<K, V> {
    #[allow(non_snake_case)]
    #[inline]
    fn buff_size(N: usize) -> usize {
        size_of::<V>() * (N + 1)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn file_size(N: usize) -> u64 {
        let offset = size_of::<V>() as u64 - SparSet::<K>::file_size(N) % size_of::<V>() as u64;
        SparSet::<K>::file_size(N) + offset + Self::buff_size(N) as u64
    }

    #[inline]
    pub(in crate::map) const fn from_raw(keys: SparSet<K>, vals: BufMut<V>) -> Self
    where
        V: Copy,
    {
        Self { keys, vals }
    }

    #[allow(non_snake_case)]
    pub fn from_buf(N: usize, file: File) -> Self
    where
        V: Copy,
    {
        let keys = SparSet::<K>::from_buf(N, file.try_clone().unwrap());

        let l = Self::buff_size(N);

        let vals =
            BufMut::<V>::new(&file, Self::file_size(N) - Self::buff_size(N) as u64, l).unwrap();
        debug_assert_eq!(vals.len(), N + 1);
        vals.0
            .advise_range(
                memmap2::Advice::WillNeed,
                0,
                l.min(crate::MAX_BYTE_PRE_LOAD_SIZE),
            )
            .unwrap();
        vals.0.advise(memmap2::Advice::Sequential).unwrap();

        Self::from_raw(keys, vals)
    }
}

impl<K, V> ModelRefAccess<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn k(&self) -> &SparSet<K> {
        &self.keys
    }

    #[inline]
    fn v(&self) -> &[V] {
        &self.vals
    }

    #[inline]
    fn kv(&self) -> (&SparSet<K>, &[V]) {
        (&self.keys, &self.vals)
    }
}

impl<K, V> ModelMutAccess<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn k(&mut self) -> &mut SparSet<K> {
        &mut self.keys
    }

    #[inline]
    fn v(&mut self) -> &mut [V] {
        &mut self.vals
    }

    #[inline]
    fn kv(&mut self) -> (&mut SparSet<K>, &mut [V]) {
        (&mut self.keys, &mut self.vals)
    }
}
