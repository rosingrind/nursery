use core::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Sub, SubAssign};

use super::{Key, SetMut, SetRef, SparSet};

impl PartialEq for SparSet {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|&key| other.contains(key))
    }
}

impl Eq for SparSet {}

impl fmt::Debug for SparSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}

impl FromIterator<Key> for SparSet {
    #[cfg_attr(feature = "inline-more", inline)]
    fn from_iter<I: IntoIterator<Item = Key>>(iter: I) -> Self {
        let mut set = Self::new();
        set.extend(iter);
        set
    }
}

impl<const N: usize> From<[Key; N]> for SparSet {
    fn from(arr: [Key; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl Extend<Key> for SparSet {
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = Key>>(&mut self, iter: I) {
        iter.into_iter().for_each(|k| {
            self.insert_one(k);
        });
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, k: Key) {
        self.insert_one(k);
    }
}

impl<'a> Extend<&'a Key> for SparSet {
    #[cfg_attr(feature = "inline-more", inline)]
    fn extend<I: IntoIterator<Item = &'a Key>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "nightly")]
    fn extend_one(&mut self, k: &'a Key) {
        self.insert_one(*k);
    }
}

impl BitOr<&SparSet> for &SparSet {
    type Output = SparSet;

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
    fn bitor(self, rhs: &SparSet) -> SparSet {
        self.union(rhs).cloned().collect()
    }
}

impl BitAnd<&SparSet> for &SparSet {
    type Output = SparSet;

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
    fn bitand(self, rhs: &SparSet) -> SparSet {
        self.intersection(rhs).cloned().collect()
    }
}

impl BitXor<&SparSet> for &SparSet {
    type Output = SparSet;

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
    fn bitxor(self, rhs: &SparSet) -> SparSet {
        self.symmetric_difference(rhs).cloned().collect()
    }
}

impl Sub<&SparSet> for &SparSet {
    type Output = SparSet;

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
    fn sub(self, rhs: &SparSet) -> SparSet {
        self.difference(rhs).cloned().collect()
    }
}

impl BitOrAssign<&SparSet> for SparSet {
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
    fn bitor_assign(&mut self, rhs: &SparSet) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            }
        }
    }
}

impl BitAndAssign<&SparSet> for SparSet {
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
    fn bitand_assign(&mut self, rhs: &SparSet) {
        self.retain(|&item| rhs.contains(item));
    }
}

impl BitXorAssign<&SparSet> for SparSet {
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
    fn bitxor_assign(&mut self, rhs: &SparSet) {
        for item in rhs.iter().copied() {
            if !self.contains(item) {
                self.insert_one(item);
            } else {
                self.delete_one(item);
            }
        }
    }
}

impl SubAssign<&SparSet> for SparSet {
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
    fn sub_assign(&mut self, rhs: &SparSet) {
        if rhs.len() < self.len() {
            for item in rhs.iter().copied() {
                self.delete_one(item);
            }
        } else {
            self.retain(|&item| !rhs.contains(item));
        }
    }
}
