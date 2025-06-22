mod difference;
mod intersection;
mod symmetric_difference;
mod union;

use std::slice::Iter;

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub(super) use difference::*;
pub(super) use intersection::*;
pub(super) use symmetric_difference::*;
pub(super) use union::*;

use super::SparSet;

type SetIter<'a, K> = Iter<'a, K>;

pub trait SetRef<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Returns raw dense indexes slice
    fn as_slice(&self) -> &[K];

    fn iter(&self) -> SetIter<K>;

    #[cfg(not(feature = "rayon"))]
    /// A ∩ B
    fn intersection<'a>(&'a self, other: &'a SparSet<K>) -> Intersection<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∩ B (parallel)
    fn intersection(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A ∪ B
    fn union<'a>(&'a self, other: &'a SparSet<K>) -> Union<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∪ B (parallel)
    fn union(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn difference<'a>(&'a self, other: &'a SparSet<K>) -> Difference<'a, K>;
    #[cfg(feature = "rayon")]
    /// A − B (parallel)
    fn difference(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn symmetric_difference<'a>(&'a self, other: &'a SparSet<K>) -> SymmetricDifference<'a, K>;

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
    fn intersection<'a>(&'a self, other: &'a SparSet<K>) -> Intersection<'a, K> {
        // let (smaller, larger) = if self.len() <= other.len() {
        //     (self, other)
        // } else {
        //     (other, self)
        // };
        Intersection {
            iter: self.iter(),
            other: other,
        }
    }

    #[cfg(feature = "rayon")]
    fn intersection(&self, other: &Self) -> impl SetRef {
        todo!()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn union<'a>(&'a self, other: &'a SparSet<K>) -> Union<'a, K> {
        // let (smaller, larger) = if self.len() <= other.len() {
        //     (self, other)
        // } else {
        //     (other, self)
        // };
        Union {
            iter: other.iter().chain(self.difference(other)),
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    fn union(&self, other: &Self) -> impl SetRef {
        todo!()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn difference<'a>(&'a self, other: &'a SparSet<K>) -> Difference<'a, K> {
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
    fn symmetric_difference<'a>(&'a self, other: &'a SparSet<K>) -> SymmetricDifference<'a, K> {
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
