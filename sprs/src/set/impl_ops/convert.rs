use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SparSet};

impl<K> FromIterator<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Ord,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let arr: Box<[K]> = iter.into_iter().collect();
        let mut set = Self::new(arr.iter().max().unwrap().as_());
        set.extend(arr);

        set
    }
}

impl<K> Extend<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = K>>(&mut self, iter: I) {
        self.insert_all(iter);
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, k: K) {
        self.insert_one(k);
    }
}

impl<'a, K> Extend<&'a K> for SparSet<K>
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
