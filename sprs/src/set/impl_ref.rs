use core::fmt;
use std::{
    fmt::Debug,
    iter::{Chain, FusedIterator},
    slice::Iter,
};

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::SparSet;

pub(crate) type SetIter<'a, K> = Iter<'a, K>;

pub trait SetRef<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Returns raw dense indexes slice
    fn as_slice(&self) -> &[K];

    fn iter(&self) -> SetIter<K>;

    #[cfg(not(feature = "rayon"))]
    /// A ∩ B
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∩ B (parallel)
    fn intersection(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A ∪ B
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∪ B (parallel)
    fn union(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K>;
    #[cfg(feature = "rayon")]
    /// A − B (parallel)
    fn difference(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K>;

    fn is_disjoint(&self, other: &Self) -> bool;

    fn is_subset(&self, other: &Self) -> bool;

    fn is_superset(&self, other: &Self) -> bool;
}

impl<K> SetRef<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn as_slice(&self) -> &[K] {
        &self.dense[..self.len.as_()]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn iter(&self) -> SetIter<K> {
        self.dense[..self.len.as_()].iter()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K> {
        let (smaller, larger) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        Intersection {
            iter: smaller.iter(),
            other: larger,
        }
    }

    #[cfg(feature = "rayon")]
    fn intersection(&self, other: &Self) -> impl SetRef {
        todo!()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K> {
        let (smaller, larger) = if self.len() <= other.len() {
            (self, other)
        } else {
            (other, self)
        };
        Union {
            iter: larger.iter().chain(smaller.difference(larger)),
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    fn union(&self, other: &Self) -> impl SetRef {
        todo!()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    fn difference(&self, other: &Self) -> impl SetRef {
        todo!()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
        // if self.len() <= other.len() {
        //     self.iter().all(|v| !other.contains(*v))
        // } else {
        //     other.iter().all(|v| !self.contains(*v))
        // }
    }

    fn is_subset(&self, other: &Self) -> bool {
        self.len() <= other.len() && self.iter().all(|&v| other.contains(v))
        // if self.len() <= other.len() {
        //     self.iter().all(|v| other.contains(*v))
        // } else {
        //     false
        // }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }
}

pub struct Intersection<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    iter: Iter<'a, K>,
    other: &'a SparSet<K>,
}

pub struct Union<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    iter: Chain<Iter<'a, K>, Difference<'a, K>>,
}

pub struct Difference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    iter: Iter<'a, K>,
    other: &'a SparSet<K>,
}

pub struct SymmetricDifference<'a, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    iter: Chain<Difference<'a, K>, Difference<'a, K>>,
}

impl<K> Clone for Intersection<'_, K>
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

impl<'a, K> Iterator for Intersection<'a, K>
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

impl<K> fmt::Debug for Intersection<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K> FusedIterator for Intersection<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> ExactSizeIterator for Intersection<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> Clone for Union<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<'a, K> Iterator for Union<'a, K>
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

impl<K> fmt::Debug for Union<'_, K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<K> FusedIterator for Union<'_, K> where K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd {}

impl<K> ExactSizeIterator for Union<'_, K> where K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{}

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

impl<K> FusedIterator for Difference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> ExactSizeIterator for Difference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
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

impl<K> FusedIterator for SymmetricDifference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}

impl<K> ExactSizeIterator for SymmetricDifference<'_, K> where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd
{
}
