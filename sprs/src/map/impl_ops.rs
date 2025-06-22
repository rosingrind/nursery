use std::{
    fmt::{self, Debug},
    ops::Index,
};

use num_traits::{AsPrimitive, Unsigned};

use super::{SparMap, impl_mut::MapMut, impl_ref::MapRef};

impl<K, V> PartialEq for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy + PartialEq,
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

impl<K, V> Eq for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy + PartialEq,
{
}

impl<K, V> Debug for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
    V: Send + Sync + Copy + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> FromIterator<(K, V)> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let iter = iter.into_iter();
        let (lower, upper) = iter.size_hint();

        let mut map = Self::new(upper.unwrap_or(lower));
        map.extend(iter);
        map
    }
}

impl<K, V> From<&[(K, V)]> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    fn from(arr: &[(K, V)]) -> Self {
        arr.into_iter().cloned().collect()
    }
}

impl<K, V> Extend<(K, V)> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        iter.into_iter().for_each(|(k, v)| {
            self.insert_one(k, v);
        });
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (K, V)) {
        self.insert_one(k, v);
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (&'a K, &'a V)) {
        self.insert_one(*k, *v);
    }
}

impl<'a, K, V> Extend<&'a (K, V)> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = &'a (K, V)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|&(key, value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, &(k, v): &'a (K, V)) {
        self.insert_one(k, v);
    }
}

impl<K, V> Index<K> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
    V: Send + Sync + Copy,
{
    type Output = V;

    #[cfg_attr(feature = "inline-more", inline)]
    fn index(&self, key: K) -> &V {
        self.query_one(key).expect("no entry found for key")
    }
}
