use num_traits::{AsPrimitive, Unsigned};

#[cfg(feature = "volatile")]
mod default;
#[cfg(feature = "memmap2")]
mod memmap2;

#[cfg(feature = "volatile")]
pub use default::*;
#[cfg(feature = "memmap2")]
pub use memmap2::*;

use crate::set::SparSet;

pub(super) trait ModelRefAccess<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Get model's keys as `&`
    fn k(&self) -> &SparSet<K>;
    /// Get model's vals as `&`
    fn v(&self) -> &[V];
    #[allow(dead_code)]
    /// Get model's `(keys, vals)` tuple as `(&, &)`
    fn kv(&self) -> (&SparSet<K>, &[V]);
}

pub(super) trait ModelMutAccess<K, V>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Get model's keys as `&mut `
    fn k(&mut self) -> &mut SparSet<K>;
    /// Get model's vals as `&mut `
    fn v(&mut self) -> &mut [V];
    /// Get model's `(keys, vals)` tuple as `(&mut, &mut)`
    fn kv(&mut self) -> (&mut SparSet<K>, &mut [V]);
}
