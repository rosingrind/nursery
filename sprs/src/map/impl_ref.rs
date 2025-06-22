use std::{iter::Zip, slice::Iter};

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::SparMap;

pub(super) type MapIter<'a, K, V> = Zip<Iter<'a, K>, Iter<'a, V>>;

pub trait MapRef<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn iter(&self) -> MapIter<K, V>;
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
}
