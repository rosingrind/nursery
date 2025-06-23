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
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Ord,
    V: Send + Sync + Copy,
{
    #[cfg_attr(feature = "inline-more", inline)]
    // TODO: get max element from iterator without consuming and construct Self
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let arr: Vec<(K, V)> = iter.into_iter().collect();
        Self::from(arr)
    }
}

impl<K, V> From<Vec<(K, V)>> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Ord,
    V: Send + Sync + Copy,
{
    /// Portable [`SparMap::insert_all`] implementation, subject to change
    fn from(arr: Vec<(K, V)>) -> Self {
        let mut map = Self::new(arr.iter().max_by_key(|(k, _)| k).unwrap().0.as_());
        let (k, v) = map.filter_all_excl(arr);

        let len = map.len().as_();
        map.keys.insert_all_seq_uncheck(&k);
        map.vals[len..(len + v.len())].copy_from_slice(v.as_slice());
        map
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
