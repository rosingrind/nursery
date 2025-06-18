use core::fmt;
use num_traits::{AsPrimitive, Unsigned};
use std::{fmt::Debug, iter};

pub struct SymmetricDifference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    pub(super) iter: iter::Chain<super::Difference<'a, K>, super::Difference<'a, K>>,
}

impl<K> Clone for SymmetricDifference<'_, K>
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

impl<'a, K> Iterator for SymmetricDifference<'a, K>
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

impl<K> fmt::Debug for SymmetricDifference<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K> iter::FusedIterator for SymmetricDifference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> iter::ExactSizeIterator for SymmetricDifference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}
