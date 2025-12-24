#[cfg(feature = "rayon")]
mod par;
#[cfg(not(feature = "rayon"))]
mod seq;

use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, SubAssign};

use num_traits::{AsPrimitive, Unsigned};

use crate::set::{SetMut, SetRef, model::*};

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
    /// let mut a: SparSet<u16> = (3..=5).collect();
    /// let b: SparSet<u16> = (1..=3).collect();
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
    /// let a: SparSet<u16> = (3..=5).collect();
    /// let mut b: SparSet<u16> = (1..=3).collect();
    ///
    /// b |= &a;
    /// ```
    fn bitor_assign(&mut self, rhs: &'a SparSet<K>) {
        assert!(self.s().len() >= rhs.s().len());
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
    /// let mut a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (2..=4).collect();
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
    /// let mut a: SparSet<u16> = (3..=5).collect();
    /// let b: SparSet<u16> = (1..=3).collect();
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
    /// let a: SparSet<u16> = (3..=5).collect();
    /// let mut b: SparSet<u16> = (1..=3).collect();
    ///
    /// b ^= &a;
    /// ```
    fn bitxor_assign(&mut self, rhs: &'a SparSet<K>) {
        assert!(self.s().len() >= rhs.s().len());
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
    /// let mut a: SparSet<u16> = (1..=3).collect();
    /// let b: SparSet<u16> = (3..=5).collect();
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
