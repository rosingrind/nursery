mod buf_mut;
mod val_mut;

use std::fs::File;

use buf_mut::*;
use val_mut::*;

use num_traits::{AsPrimitive, Unsigned};

use super::{ModelMutAccess, ModelRefAccess};

pub struct SparSet<K: Unsigned> {
    len: ValMut<K>,
    buf_s: BufMut<K>,
    buf_d: BufMut<K>,
}

impl<K: Unsigned> SparSet<K> {
    #[inline]
    pub(in crate::set) fn from_raw(len: ValMut<K>, buf_s: BufMut<K>, buf_d: BufMut<K>) -> Self {
        Self { len, buf_s, buf_d }
    }

    #[cfg(feature = "memmap2")]
    #[allow(non_snake_case)]
    pub fn from_buf(N: usize, file: File) -> Self {
        assert!(N <= Self::MAX_K);

        let len = ValMut::<K>::new(&file, size_of::<K>(), 0).unwrap();
        len.0.advise(memmap2::Advice::WillNeed).unwrap();

        let l = size_of::<K>() * (N + 1);

        let buf_s = BufMut::<K>::new(&file, l, size_of::<K>() as u64).unwrap();
        buf_s.0.advise(memmap2::Advice::Sequential).unwrap();
        debug_assert_eq!(buf_s.len(), N + 1);

        let buf_d = BufMut::<K>::new(&file, l, (size_of::<K>() + l) as u64).unwrap();
        buf_d.0.advise(memmap2::Advice::Sequential).unwrap();
        debug_assert_eq!(buf_d.len(), N + 1);

        assert_eq!(buf_s.len(), buf_d.len());

        Self::from_raw(len, buf_s, buf_d)
    }
}

impl<K> ModelRefAccess<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn l(&self) -> &K {
        &self.len
    }

    #[inline]
    fn s(&self) -> &[K] {
        &self.buf_s
    }

    #[inline]
    fn d(&self) -> &[K] {
        &self.buf_d
    }
}

impl<K> ModelMutAccess<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn l(&mut self) -> &mut K {
        &mut self.len
    }

    #[inline]
    fn s(&mut self) -> &mut [K] {
        &mut self.buf_s
    }

    #[inline]
    fn d(&mut self) -> &mut [K] {
        &mut self.buf_d
    }
}
