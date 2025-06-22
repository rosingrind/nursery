use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SparSet};

impl<K, const N: usize> FromIterator<K> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<K, const N: usize> From<[K; N]> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn from(arr: [K; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, const N: usize> Extend<K> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
        iter.into_iter().for_each(|k| {
            self.insert_one(k);
        });
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, k: K) {
        self.insert_one(k);
    }
}

impl<'a, K, const N: usize> Extend<&'a K> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = &'a K>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, k: &'a K) {
        self.insert_one(*k);
    }
}
