use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SetRef, SparSet};

impl<K, const N: usize> BitOr<&SparSet<K, N>> for &SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K, N>;

    /// Returns the union of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a | &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 3, 4, 5];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitor(self, rhs: &SparSet<K, N>) -> SparSet<K, N> {
        self.union(rhs).cloned().collect()
    }
}

impl<K, const N: usize> BitAnd<&SparSet<K, N>> for &SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K, N>;

    /// Returns the intersection of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![2, 3, 4].into_iter().collect();
    ///
    /// let set = &a & &b;
    ///
    /// let mut i = 0;
    /// let expected = [2, 3];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitand(self, rhs: &SparSet<K, N>) -> SparSet<K, N> {
        self.intersection(rhs).cloned().collect()
    }
}

impl<K, const N: usize> BitXor<&SparSet<K, N>> for &SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K, N>;

    /// Returns the symmetric difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a ^ &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2, 4, 5];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn bitxor(self, rhs: &SparSet<K, N>) -> SparSet<K, N> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<K, const N: usize> Sub<&SparSet<K, N>> for &SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K, N>;

    /// Returns the difference of `self` and `rhs` as a new `SparSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
    ///
    /// let set = &a - &b;
    ///
    /// let mut i = 0;
    /// let expected = [1, 2];
    /// for x in &set {
    ///     assert!(expected.contains(x));
    ///     i += 1;
    /// }
    /// assert_eq!(i, expected.len());
    /// ```
    fn sub(self, rhs: &SparSet<K, N>) -> SparSet<K, N> {
        self.difference(rhs).cloned().collect()
    }
}

impl<K, const N: usize> BitOrAssign<&SparSet<K, N>> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the union of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let mut a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
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
    fn bitor_assign(&mut self, rhs: &SparSet<K, N>) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            }
        }
    }
}

impl<K, const N: usize> BitAndAssign<&SparSet<K, N>> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the intersection of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let mut a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![2, 3, 4].into_iter().collect();
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
    fn bitand_assign(&mut self, rhs: &SparSet<K, N>) {
        self.retain(|&item| rhs.contains(item));
    }
}

impl<K, const N: usize> BitXorAssign<&SparSet<K, N>> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the symmetric difference of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let mut a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
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
    fn bitxor_assign(&mut self, rhs: &SparSet<K, N>) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            } else {
                self.delete_one(item);
            }
        }
    }
}

impl<K, const N: usize> SubAssign<&SparSet<K, N>> for SparSet<K, N>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Modifies this set to contain the difference of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// use spars::set::SparSet;
    ///
    /// let mut a: SparSet = vec![1, 2, 3].into_iter().collect();
    /// let b: SparSet = vec![3, 4, 5].into_iter().collect();
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
    fn sub_assign(&mut self, rhs: &SparSet<K, N>) {
        if rhs.len() < self.len() {
            for item in rhs.iter().copied() {
                self.delete_one(item);
            }
        } else {
            self.retain(|&item| !rhs.contains(item));
        }
    }
}
