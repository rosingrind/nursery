use std::{iter::Zip, slice::Iter};

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::SparMap;

pub(super) type MapIter<'a, K, T> = Zip<Iter<'a, K>, Iter<'a, T>>;

pub trait MapRef<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn iter(&self) -> MapIter<K, T>;
}

impl<K, T> MapRef<K, T> for SparMap<K, T>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn iter(&self) -> MapIter<K, T> {
        use crate::set::SetRef;

        self.keys.iter().zip(self.vals.iter())
    }
}
