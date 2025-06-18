use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SetRef, SparSet};

impl<K> BitOr<&SparSet<K>> for &SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K>;

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
    fn bitor(self, rhs: &SparSet<K>) -> SparSet<K> {
        self.union(rhs).cloned().collect()
    }
}

impl<K> BitAnd<&SparSet<K>> for &SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K>;

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
    fn bitand(self, rhs: &SparSet<K>) -> SparSet<K> {
        self.intersection(rhs).cloned().collect()
    }
}

impl<K> BitXor<&SparSet<K>> for &SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K>;

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
    fn bitxor(self, rhs: &SparSet<K>) -> SparSet<K> {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl<K> Sub<&SparSet<K>> for &SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    type Output = SparSet<K>;

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
    fn sub(self, rhs: &SparSet<K>) -> SparSet<K> {
        self.difference(rhs).cloned().collect()
    }
}

impl<K> BitOrAssign<&SparSet<K>> for SparSet<K>
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
    fn bitor_assign(&mut self, rhs: &SparSet<K>) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            }
        }
    }
}

impl<K> BitAndAssign<&SparSet<K>> for SparSet<K>
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
    fn bitand_assign(&mut self, rhs: &SparSet<K>) {
        self.retain(|&item| rhs.contains(item));
    }
}

impl<K> BitXorAssign<&SparSet<K>> for SparSet<K>
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
    fn bitxor_assign(&mut self, rhs: &SparSet<K>) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            } else {
                self.delete_one(item);
            }
        }
    }
}

impl<K> SubAssign<&SparSet<K>> for SparSet<K>
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
    fn sub_assign(&mut self, rhs: &SparSet<K>) {
        if rhs.len() < self.len() {
            for item in rhs.iter().copied() {
                self.delete_one(item);
            }
        } else {
            self.retain(|&item| !rhs.contains(item));
        }
    }
}
