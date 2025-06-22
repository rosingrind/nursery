use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{
    SetMut, SetRef, SparSet,
    impl_ref::{Difference, Intersection, SymmetricDifference, Union},
};

impl<'a, K, const N: usize, const M: usize> BitOr<&'a SparSet<K, M>> for &'a SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Union<'a, K, N, M>;

    /// Returns the union of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn bitor(self, rhs: &'a SparSet<K, M>) -> Self::Output {
        self.union(rhs)
    }
}

impl<'a, K, const N: usize, const M: usize> BitAnd<&'a SparSet<K, M>> for &'a SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Intersection<'a, K, N, M>;

    /// Returns the intersection of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 4> = vec![2, 3, 4].into_iter().collect();
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
    fn bitand(self, rhs: &'a SparSet<K, M>) -> Self::Output {
        self.intersection(rhs)
    }
}

impl<'a, K, const N: usize, const M: usize> BitXor<&'a SparSet<K, M>> for &'a SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SymmetricDifference<'a, K, N, M>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn bitxor(self, rhs: &'a SparSet<K, M>) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl<'a, K, const N: usize, const M: usize> Sub<&'a SparSet<K, M>> for &'a SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = Difference<'a, K, N, M>;

    /// Returns the difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sprs::set::SparSet;
    ///
    /// let a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn sub(self, rhs: &'a SparSet<K, M>) -> Self::Output {
        self.difference(rhs)
    }
}

impl<'a, K, const N: usize, const M: usize> BitOrAssign<&'a SparSet<K, M>> for SparSet<K, N>
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
    /// let mut a: SparSet<u16, 5> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn bitor_assign(&mut self, rhs: &'a SparSet<K, M>) {
        assert!(N >= M);
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            }
        }
    }
}

impl<'a, K, const N: usize, const M: usize> BitAndAssign<&'a SparSet<K, M>> for SparSet<K, N>
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
    /// let mut a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 4> = vec![2, 3, 4].into_iter().collect();
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
    fn bitand_assign(&mut self, rhs: &'a SparSet<K, M>) {
        self.retain(|&item| rhs.contains(item));
    }
}

impl<'a, K, const N: usize, const M: usize> BitXorAssign<&'a SparSet<K, M>> for SparSet<K, N>
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
    /// let mut a: SparSet<u16, 5> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn bitxor_assign(&mut self, rhs: &'a SparSet<K, M>) {
        assert!(N >= M);
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            } else {
                self.delete_one(item);
            }
        }
    }
}

impl<'a, K, const N: usize, const M: usize> SubAssign<&'a SparSet<K, M>> for SparSet<K, N>
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
    /// let mut a: SparSet<u16, 3> = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet<u16, 5> = vec![3, 4, 5].into_iter().collect();
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
    fn sub_assign(&mut self, rhs: &'a SparSet<K, M>) {
        if rhs.len() < self.len() {
            for item in rhs.iter().copied() {
                self.delete_one(item);
            }
        } else {
            self.retain(|&item| !rhs.contains(item));
        }
    }
}
