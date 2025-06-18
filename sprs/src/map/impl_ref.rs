use std::{iter::Zip, slice::Iter};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::{Key, SparMap};

pub(crate) type MapIter<'a, T> = Zip<Iter<'a, Key>, Iter<'a, T>>;

pub trait MapRef<T> {
    fn iter(&self) -> MapIter<T>;
}

impl<T> MapRef<T> for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn iter(&self) -> MapIter<T> {
        use crate::set::SetRef;

        self.keys.iter().zip(self.vals.iter())
    }
}
