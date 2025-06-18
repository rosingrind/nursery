mod convert;
mod set_ops;

use std::fmt::{self, Debug};

use num_traits::{AsPrimitive, Unsigned};

use super::{SetRef, SparSet};

impl<K> PartialEq for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter().all(|&key| other.contains(key))
    }
}

impl<K> Eq for SparSet<K> where K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd {}

impl<K> Debug for SparSet<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_set().entries(self.iter()).finish()
    }
}
