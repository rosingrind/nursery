use core::fmt;
use num_traits::{AsPrimitive, Unsigned};
use std::{fmt::Debug, iter, slice::Iter};

use crate::set::SparSet;

pub struct Difference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(in crate::set) iter: Iter<'a, K>,
    pub(in crate::set) other: &'a SparSet<K>,
}

impl<K> Clone for Difference<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Difference {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a, K> Iterator for Difference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a K> {
        loop {
            let elt = self.iter.next()?;
            if !self.other.contains(*elt) {
                return Some(elt);
            }
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower.saturating_sub(self.other.len().as_()), upper)
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, elt| {
            if self.other.contains(*elt) {
                acc
            } else {
                f(acc, elt)
            }
        })
    }
}

impl<K> fmt::Debug for Difference<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K> iter::FusedIterator for Difference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> iter::ExactSizeIterator for Difference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}
