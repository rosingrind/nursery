mod difference;
mod intersection;
mod symmetric_difference;
mod union;

use num_traits::{AsPrimitive, Unsigned};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub(super) use difference::*;
pub(super) use intersection::*;
pub(super) use symmetric_difference::*;
pub(super) use union::*;

use super::{SparSet, model::*};

pub type SetIter<'a, K> = std::slice::Iter<'a, K>;
#[cfg(feature = "rayon")]
pub type SetParIter<'a, K> = rayon::slice::Iter<'a, K>;

pub trait SetRef<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Returns raw dense indexes slice
    fn as_slice(&self) -> &[K];

    fn iter(&self) -> SetIter<K>;

    #[cfg(feature = "rayon")]
    fn par_iter(&self) -> SetParIter<K>
    where
        K: Sync;

    #[cfg(not(feature = "rayon"))]
    /// A ∩ B
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∩ B (parallel)
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K>;

    #[cfg(not(feature = "rayon"))]
    /// A ∪ B
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K>;
    #[cfg(feature = "rayon")]
    /// A ∪ B (parallel)
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K>;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K>;
    #[cfg(feature = "rayon")]
    /// A − B (parallel)
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K>;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K>;
    #[cfg(feature = "rayon")]
    /// A − B (parallel)
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K>;

    #[cfg(not(feature = "rayon"))]
    fn is_disjoint(&self, other: &Self) -> bool;

    #[cfg(feature = "rayon")]
    fn is_disjoint(&self, other: &Self) -> bool
    where
        K: Sync;

    #[cfg(not(feature = "rayon"))]
    fn is_subset(&self, other: &Self) -> bool;

    #[cfg(feature = "rayon")]
    fn is_subset(&self, other: &Self) -> bool
    where
        K: Sync;

    #[cfg(not(feature = "rayon"))]
    fn is_superset(&self, other: &Self) -> bool;

    #[cfg(feature = "rayon")]
    fn is_superset(&self, other: &Self) -> bool
    where
        K: Sync;
}

impl<K> SetRef<K> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn as_slice(&self) -> &[K] {
        &self.d()[..self.l().as_()]
    }

    fn iter(&self) -> SetIter<K> {
        self.d()[..self.l().as_()].iter()
    }

    #[cfg(feature = "rayon")]
    fn par_iter(&self) -> SetParIter<K>
    where
        K: Sync,
    {
        self.dense()[..self.len().as_()].par_iter()
    }

    #[cfg(not(feature = "rayon"))]
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K> {
        let (smaller, larger) =
            std::hint::select_unpredictable(self.l() <= other.l(), (self, other), (other, self));
        Intersection {
            iter: smaller.iter(),
            other: larger,
        }
    }

    #[cfg(feature = "rayon")]
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a, K> {
        Intersection { a: self, b: other }
    }

    #[cfg(not(feature = "rayon"))]
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K> {
        let (smaller, larger) =
            std::hint::select_unpredictable(self.l() <= other.l(), (self, other), (other, self));
        Union {
            iter: larger.iter().chain(smaller.difference(larger)),
        }
    }

    #[cfg(feature = "rayon")]
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a, K> {
        Union { a: self, b: other }
    }

    #[cfg(not(feature = "rayon"))]
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K> {
        Difference {
            iter: self.iter(),
            other,
        }
    }

    #[cfg(feature = "rayon")]
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a, K> {
        Difference { a: self, b: other }
    }

    #[cfg(not(feature = "rayon"))]
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K> {
        SymmetricDifference {
            iter: self.difference(other).chain(other.difference(self)),
        }
    }

    #[cfg(feature = "rayon")]
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a, K> {
        SymmetricDifference { a: self, b: other }
    }

    #[cfg(not(feature = "rayon"))]
    fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).next().is_none()
        // if self.len() <= other.len() {
        //     self.iter().all(|v| !other.contains(*v))
        // } else {
        //     other.iter().all(|v| !self.contains(*v))
        // }
    }

    #[cfg(feature = "rayon")]
    fn is_disjoint(&self, other: &Self) -> bool
    where
        K: Sync,
    {
        <Self as SetRef<K>>::par_iter(self).all(|x| !other.contains(*x))
    }

    #[cfg(not(feature = "rayon"))]
    fn is_subset(&self, other: &Self) -> bool {
        self.l() <= other.l() && self.iter().all(|&v| other.contains(v))
        // if self.len() <= other.len() {
        //     self.iter().all(|v| other.contains(*v))
        // } else {
        //     false
        // }
    }

    #[cfg(feature = "rayon")]
    fn is_subset(&self, other: &Self) -> bool
    where
        K: Sync,
    {
        if self.len() <= other.len() {
            <Self as SetRef<K>>::par_iter(self).all(|x| other.contains(*x))
        } else {
            false
        }
    }

    #[cfg(not(feature = "rayon"))]
    fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    #[cfg(feature = "rayon")]
    fn is_superset(&self, other: &Self) -> bool
    where
        K: Sync,
    {
        other.is_subset(self)
    }
}
