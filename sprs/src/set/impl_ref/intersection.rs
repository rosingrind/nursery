use core::fmt;
use num_traits::{AsPrimitive, Unsigned};
use std::{fmt::Debug, iter, slice::Iter};

use crate::set::SparSet;

pub struct Intersection<'a, K, const N: usize>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: Iter<'a, K>,
    pub(super) other: &'a SparSet<K, N>,
}

impl<K, const N: usize> Clone for Intersection<'_, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Intersection {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a, K, const N: usize> Iterator for Intersection<'a, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a K> {
        loop {
            let elt = self.iter.next()?;
            if self.other.contains(*elt) {
                return Some(elt);
            }
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_, upper) = self.iter.size_hint();
        (0, upper)
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, elt| {
            if self.other.contains(*elt) {
                f(acc, elt)
            } else {
                acc
            }
        })
    }
}

impl<K, const N: usize> fmt::Debug for Intersection<'_, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K, const N: usize> iter::FusedIterator for Intersection<'_, K, N> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K, const N: usize> iter::ExactSizeIterator for Intersection<'_, K, N> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}
