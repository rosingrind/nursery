use core::fmt;
use num_traits::{AsPrimitive, Unsigned};
use std::{fmt::Debug, iter};

pub struct SymmetricDifference<'a, K, const N: usize>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: iter::Chain<super::Difference<'a, K, N>, super::Difference<'a, K, N>>,
}

impl<K, const N: usize> Clone for SymmetricDifference<'_, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K, const N: usize> Iterator for SymmetricDifference<'a, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Item = &'a K;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a K> {
        self.iter.next()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, f)
    }
}

impl<K, const N: usize> fmt::Debug for SymmetricDifference<'_, K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K, const N: usize> iter::FusedIterator for SymmetricDifference<'_, K, N> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K, const N: usize> iter::ExactSizeIterator for SymmetricDifference<'_, K, N> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}
