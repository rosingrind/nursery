use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{
    SetMut, SetRef, SparSet,
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
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([3, 4, 5].as_slice());
    ///
    /// let set = &a | &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 3, 4, 5];
    /// for x in set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
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
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([2, 3, 4].as_slice());
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
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
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([3, 4, 5].as_slice());
    ///
    /// let set = &a ^ &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 4, 5];
    /// for x in set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
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
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([3, 4, 5].as_slice());
    ///
    /// let set = &a - &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2];
    /// for x in set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn sub(self, rhs: &'a SparSet<K>) -> Self::Output {
        self.difference(rhs)
    }
}

impl<'a, K> BitOrAssign<&'a SparSet<K>> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the union of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let mut a: SparSet<u16> = From::from([3, 4, 5].as_slice());
    /// let b: SparSet<u16> = From::from([1, 2, 3].as_slice());
    ///
    /// a |= &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 3, 4, 5];
    /// for x in &a {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    ///
    /// Notice that this panics with `index out of bounds: the len is 4 but the index is 4`:
    ///
    /// ```should_panic
    /// # use sprs::set::SparSet;
    /// #
    /// let a: SparSet<u16> = From::from([3, 4, 5].as_slice());
    /// let mut b: SparSet<u16> = From::from([1, 2, 3].as_slice());
    ///
    /// b |= &a;
    /// ```
    fn bitor_assign(&mut self, rhs: &'a SparSet<K>) {
        assert!(self.sparse.len() >= rhs.sparse.len());
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            }
        }
    }
}

impl<'a, K> BitAndAssign<&'a SparSet<K>> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the intersection of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let mut a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([2, 3, 4].as_slice());
    ///
    /// a &= &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &a {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand_assign(&mut self, rhs: &'a SparSet<K>) {
        self.retain(|&item| rhs.contains(item));
    }
}

impl<'a, K> BitXorAssign<&'a SparSet<K>> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the symmetric difference of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let mut a: SparSet<u16> = From::from([3, 4, 5].as_slice());
    /// let b: SparSet<u16> = From::from([1, 2, 3].as_slice());
    ///
    /// a ^= &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 4, 5];
    /// for x in &a {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    ///
    /// Notice that this panics with `index out of bounds: the len is 4 but the index is 4`:
    ///
    /// ```should_panic
    /// # use sprs::set::SparSet;
    /// #
    /// let a: SparSet<u16> = From::from([3, 4, 5].as_slice());
    /// let mut b: SparSet<u16> = From::from([1, 2, 3].as_slice());
    ///
    /// b ^= &a;
    /// ```
    fn bitxor_assign(&mut self, rhs: &'a SparSet<K>) {
        assert!(self.sparse.len() >= rhs.sparse.len());
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            } else {
                self.delete_one(item);
            }
        }
    }
}

impl<'a, K> SubAssign<&'a SparSet<K>> for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the difference of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let mut a: SparSet<u16> = From::from([1, 2, 3].as_slice());
    /// let b: SparSet<u16> = From::from([3, 4, 5].as_slice());
    ///
    /// a -= &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2];
    /// for x in &a {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn sub_assign(&mut self, rhs: &'a SparSet<K>) {
        if rhs.len() < self.len() {
            for item in rhs.iter().copied() {
                self.delete_one(item);
            }
        } else {
            self.retain(|&item| !rhs.contains(item));
        }
    }
}
