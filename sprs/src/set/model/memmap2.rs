use std::fs::File;

use num_traits::{AsPrimitive, Unsigned};

use crate::ext::*;

use super::{ModelMutAccess, ModelRefAccess};

pub struct SparSet<K: Unsigned> {
    len: ValMut<K>,
    buf_s: BufMut<K>,
    buf_d: BufMut<K>,
}

impl<K: Unsigned> SparSet<K> {
    #[allow(non_snake_case)]
    #[inline]
    fn buff_size(N: usize) -> usize {
        size_of::<K>() * (N + 1)
    }

    #[allow(non_snake_case)]
    #[inline]
    pub fn file_size(N: usize) -> u64 {
        size_of::<K>() as u64 + Self::buff_size(N) as u64 * 2
    }

    #[inline]
    pub(in crate::set) const fn from_raw(
        len: ValMut<K>,
        buf_s: BufMut<K>,
        buf_d: BufMut<K>,
    ) -> Self {
        Self { len, buf_s, buf_d }
    }

    #[allow(non_snake_case)]
    pub fn from_buf(N: usize, file: File) -> Self {
        assert!(N <= Self::MAX_K);

        let len = ValMut::<K>::new(&file, 0).unwrap();
        len.0.advise(memmap2::Advice::WillNeed).unwrap();

        let l = Self::buff_size(N);

        let buf_s = BufMut::<K>::new(&file, size_of::<K>() as u64, l).unwrap();
        debug_assert_eq!(buf_s.len(), N + 1);
        buf_s
            .0
            .advise_range(
                memmap2::Advice::WillNeed,
                0,
                l.min(crate::MAX_BYTE_PRE_LOAD_SIZE),
            )
            .unwrap();
        buf_s.0.advise(memmap2::Advice::Random).unwrap();

        let buf_d = BufMut::<K>::new(&file, (size_of::<K>() + l) as u64, l).unwrap();
        debug_assert_eq!(buf_d.len(), N + 1);
        buf_d
            .0
            .advise_range(
                memmap2::Advice::WillNeed,
                0,
                l.min(crate::MAX_BYTE_PRE_LOAD_SIZE),
            )
            .unwrap();
        buf_d.0.advise(memmap2::Advice::Sequential).unwrap();

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
