use std::ops::{BitAnd, BitOr, BitXor, Sub};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{
    SetRef, SparSet,
    impl_ref::{Difference, Intersection, SymmetricDifference, Union},
};

impl<'a, K> BitOr<&'a SparSet<K>> for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Union<'a, K>;

    /// Returns the union of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprsdata::SparSet;
    ///
    /// let a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (3..=5).collect();
    ///
    /// let set = &a | &b;
    ///
    /// let expected = [1, 2, 3, 4, 5];
    /// let i = set
    ///     .inspect(|x| assert!(expected.contains(x)))
    ///     .count();
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitor(self, rhs: &'a SparSet<K>) -> Self::Output {
        self.union(rhs)
    }
}

impl<'a, K> BitAnd<&'a SparSet<K>> for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Intersection<'a, K>;

    /// Returns the intersection of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprsdata::SparSet;
    ///
    /// let a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (2..=4).collect();
    ///
    /// let set = &a & &b;
    ///
    /// let expected = [2, 3];
    /// let i = set
    ///     .inspect(|x| assert!(expected.contains(x)))
    ///     .count();
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &'a SparSet<K>) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<'a, K> BitXor<&'a SparSet<K>> for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SymmetricDifference<'a, K>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprsdata::SparSet;
    ///
    /// let a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (3..=5).collect();
    ///
    /// let set = &a ^ &b;
    ///
    /// let expected = [1, 2, 4, 5];
    /// let i = set
    ///     .inspect(|x| assert!(expected.contains(x)))
    ///     .count();
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitxor(self, rhs: &'a SparSet<K>) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl<'a, K> Sub<&'a SparSet<K>> for &'a SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Difference<'a, K>;

    /// Returns the difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprsdata::SparSet;
    ///
    /// let a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (3..=5).collect();
    ///
    /// let set = &a - &b;
    ///
    /// let expected = [1, 2];
    /// let i = set
    ///     .inspect(|x| assert!(expected.contains(x)))
    ///     .count();
    /// assert_eq!(i, expected.len());
    /// ```
    fn sub(self, rhs: &'a SparSet<K>) -> Self::Output {
        self.difference(rhs)
    }
}
