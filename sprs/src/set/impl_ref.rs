use core::fmt;
use std::{
    iter::{Chain, FusedIterator},
    slice::Iter,
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::{Key, SparSet};

pub(crate) type SetIter<'a> = Iter<'a, Key>;

pub trait SetRef {
    /// Returns raw dense indexes slice
    fn as_slice(&self) -> &[Key];

    fn iter(&self) -> SetIter;

    #[cfg(not(feature = "rayon"))]
    /// A ∩ B
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a>;
    #[cfg(feature = "rayon")]
    /// A ∩ B (parallel)
    fn intersection(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A ∪ B
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a>;
    #[cfg(feature = "rayon")]
    /// A ∪ B (parallel)
    fn union(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a>;
    #[cfg(feature = "rayon")]
    /// A − B (parallel)
    fn difference(&self, other: &Self) -> impl SetRef;

    #[cfg(not(feature = "rayon"))]
    /// A − B
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a>;

    fn is_disjoint(&self, other: &Self) -> bool;

    fn is_subset(&self, other: &Self) -> bool;

    fn is_superset(&self, other: &Self) -> bool;
}

impl SetRef for SparSet {
    #[cfg_attr(feature = "inline-more", inline)]
    fn as_slice(&self) -> &[Key] {
        &self.dense[..self.len as usize]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    fn iter(&self) -> SetIter {
        self.dense[..self.len as usize].iter()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    fn intersection<'a>(&'a self, other: &'a Self) -> Intersection<'a> {
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
    fn union<'a>(&'a self, other: &'a Self) -> Union<'a> {
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
    fn difference<'a>(&'a self, other: &'a Self) -> Difference<'a> {
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
    fn symmetric_difference<'a>(&'a self, other: &'a Self) -> SymmetricDifference<'a> {
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

pub struct Intersection<'a> {
    iter: Iter<'a, Key>,
    other: &'a SparSet,
}

pub struct Union<'a> {
    iter: Chain<Iter<'a, Key>, Difference<'a>>,
}

pub struct Difference<'a> {
    iter: Iter<'a, Key>,
    other: &'a SparSet,
}

pub struct SymmetricDifference<'a> {
    iter: Chain<Difference<'a>, Difference<'a>>,
}

impl Clone for Intersection<'_> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Intersection {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a> Iterator for Intersection<'a> {
    type Item = &'a Key;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a Key> {
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

impl fmt::Debug for Intersection<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl FusedIterator for Intersection<'_> {}

impl ExactSizeIterator for Intersection<'_> {}

impl Clone for Union<'_> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
}

impl<'a> Iterator for Union<'a> {
    type Item = &'a Key;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a Key> {
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

impl fmt::Debug for Union<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl FusedIterator for Union<'_> {}

impl ExactSizeIterator for Union<'_> {}

impl Clone for Difference<'_> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        Difference {
            iter: self.iter.clone(),
            ..*self
        }
    }
}

impl<'a> Iterator for Difference<'a> {
    type Item = &'a Key;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a Key> {
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
        (lower.saturating_sub(self.other.len() as usize), upper)
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

impl fmt::Debug for Difference<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl FusedIterator for Difference<'_> {}

impl ExactSizeIterator for Difference<'_> {}

impl Clone for SymmetricDifference<'_> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
}

impl<'a> Iterator for SymmetricDifference<'a> {
    type Item = &'a Key;

    #[cfg_attr(feature = "inline-more", inline)]
    fn next(&mut self) -> Option<&'a Key> {
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

impl fmt::Debug for SymmetricDifference<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl FusedIterator for SymmetricDifference<'_> {}

impl ExactSizeIterator for SymmetricDifference<'_> {}
