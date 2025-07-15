use num_traits::{AsPrimitive, Unsigned};

use super::{ModelMutAccess, ModelRefAccess};

#[derive(Clone)]
pub struct SparSet<K: Unsigned> {
    len: K,
    buf_s: Box<[K]>,
    buf_d: Box<[K]>,
}

impl<K: Unsigned> SparSet<K> {
    #[inline]
    pub(in crate::set) fn from_raw(len: K, buf_s: Box<[K]>, buf_d: Box<[K]>) -> Self {
        Self { len, buf_s, buf_d }
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
