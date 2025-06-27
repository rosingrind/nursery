use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::SparMap;

pub(super) type MapIter<'a, K, V> =
    std::iter::Zip<std::slice::Iter<'a, K>, std::slice::Iter<'a, V>>;
#[cfg(feature = "rayon")]
pub type MapParIter<'a, K, V> =
    rayon::iter::Zip<rayon::slice::Iter<'a, K>, rayon::slice::Iter<'a, V>>;

pub trait MapRef<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn iter(&self) -> MapIter<K, V>;

    #[cfg(feature = "rayon")]
    fn par_iter(&self) -> MapParIter<K, V>
    where
        K: Sync,
        V: Sync;
}

impl<K, V> MapRef<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn iter(&self) -> MapIter<K, V> {
        use crate::set::SetRef;

        self.keys.iter().zip(self.vals.iter())
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    fn par_iter(&self) -> MapParIter<K, V>
    where
        K: Sync,
        V: Sync,
    {
        self.keys.par_iter().zip(self.vals.par_iter())
    }
}
