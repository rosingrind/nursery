use num_traits::{AsPrimitive, Unsigned};

use crate::set::SparSet;

use super::{ModelMutAccess, ModelRefAccess};

// TOOD: sorted criteria; for example you can store `Pos { x, y }`
// as `x` and `y`, and sort each other - this allows to query
// by position (find all by `x` in range, by `y` in range, intersect results)
#[derive(Clone)]
pub struct SparMap<K: Unsigned, V> {
    keys: SparSet<K>,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: Box<[V]>,
}

impl<K: Unsigned, V> SparMap<K, V> {
    #[inline]
    pub(in crate::map) const fn from_raw(keys: SparSet<K>, vals: Box<[V]>) -> Self {
        Self { keys, vals }
    }
}

impl<K, V> ModelRefAccess<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn k(&self) -> &SparSet<K> {
        &self.keys
    }

    #[inline]
    fn v(&self) -> &[V] {
        &self.vals
    }

    #[inline]
    fn kv(&self) -> (&SparSet<K>, &[V]) {
        (&self.keys, &self.vals)
    }
}

impl<K, V> ModelMutAccess<K, V> for SparMap<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    #[inline]
    fn k(&mut self) -> &mut SparSet<K> {
        &mut self.keys
    }

    #[inline]
    fn v(&mut self) -> &mut [V] {
        &mut self.vals
    }

    #[inline]
    fn kv(&mut self) -> (&mut SparSet<K>, &mut [V]) {
        (&mut self.keys, &mut self.vals)
    }
}
