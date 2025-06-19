use std::{
    fmt::{self, Debug},
    ops::Index,
};

use num_traits::{AsPrimitive, Unsigned};

use super::{SparMap, impl_mut::MapMut, impl_ref::MapRef};

impl<K, T, const N: usize> PartialEq for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|(key, value)| {
            other
                .query_one(*key)
                .map_or_else(|| false, |v| *value == *v)
        })
    }
}

impl<K, T, const N: usize> Eq for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy + PartialEq,
{
}

impl<K, T, const N: usize> Debug for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
    T: Send + Sync + Copy + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, T, const N: usize> FromIterator<(K, T)> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = (K, T)>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<K, T, const N: usize, const M: usize> From<[(K, T); M]> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    fn from(arr: [(K, T); M]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, T, const N: usize> Extend<(K, T)> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (K, T)>>(&mut self, iter: I) {
        iter.into_iter().for_each(|(k, v)| {
            self.insert_one(k, v);
        });
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (K, T)) {
        self.insert_one(k, v);
    }
}

impl<'a, K, T, const N: usize> Extend<(&'a K, &'a T)> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (&'a K, &'a T)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (&'a K, &'a T)) {
        self.insert_one(*k, *v);
    }
}

impl<'a, K, T, const N: usize> Extend<&'a (K, T)> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = &'a (K, T)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|&(key, value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, &(k, v): &'a (K, T)) {
        self.insert_one(k, v);
    }
}

impl<K, T, const N: usize> Index<K> for SparMap<K, T, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    T: Send + Sync + Copy,
{
    type Output = T;

    #[cfg_attr(feature = "inline-more", inline)]
    fn index(&self, key: K) -> &T {
        self.query_one(key).expect("no entry found for key")
    }
}
