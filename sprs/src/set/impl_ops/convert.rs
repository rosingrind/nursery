use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SparSet};

impl<K> FromIterator<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Ord,
{
    #[cfg_attr(feature = "inline-more", inline)]
    // TODO: get max element from iterator without consuming and construct Self
    fn from_iter<I: IntoIterator<Item = K>>(iter: I) -> Self {
        let arr: Box<[K]> = iter.into_iter().collect();
        Self::from(&*arr)
    }
}

impl<K> From<&[K]> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Ord,
{
    /// Portable [`SparSet::insert_all`] implementation, subject to change
    fn from(arr: &[K]) -> Self {
        let mut set = Self::new(dbg!(arr.iter().max().unwrap().as_()));
        let s = set.filter_all_excl(arr);

        set.insert_all_seq_uncheck(&s);
        set
    }
}

impl<K> Extend<K> for SparSet<K>
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
