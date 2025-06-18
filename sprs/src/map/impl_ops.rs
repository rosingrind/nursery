use core::fmt;
use std::{fmt::Debug, ops::Index};

use crate::Key;

use super::{SparMap, impl_mut::MapMut, impl_ref::MapRef};

impl<T: Send + Sync + Copy + PartialEq> PartialEq for SparMap<T> {
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

impl<T: Send + Sync + Copy + PartialEq> Eq for SparMap<T> {}

impl<T: Send + Sync + Copy + Debug> fmt::Debug for SparMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<T: Send + Sync + Copy> FromIterator<(Key, T)> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = (Key, T)>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<T: Send + Sync + Copy, const N: usize> From<[(Key, T); N]> for SparMap<T> {
    fn from(arr: [(Key, T); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<T: Send + Sync + Copy> Extend<(Key, T)> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (Key, T)>>(&mut self, iter: I) {
        iter.into_iter().for_each(|(k, v)| {
            self.insert_one(k, v);
        });
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (Key, T)) {
        self.insert_one(k, v);
    }
}

impl<'a, T: Send + Sync + Copy> Extend<(&'a Key, &'a T)> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = (&'a Key, &'a T)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, (k, v): (&'a Key, &'a T)) {
        self.insert_one(*k, *v);
    }
}

impl<'a, T: Send + Sync + Copy> Extend<&'a (Key, T)> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = &'a (Key, T)>>(&mut self, iter: I) {
        self.extend(iter.into_iter().map(|&(key, value)| (key, value)));
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, &(k, v): &'a (Key, T)) {
        self.insert_one(k, v);
    }
}

impl<T: Send + Sync + Copy> Index<Key> for SparMap<T> {
    type Output = T;

    #[cfg_attr(feature = "inline-more", inline)]
    fn index(&self, key: Key) -> &T {
        self.query_one(key).expect("no entry found for key")
    }
}
