use num_traits::{AsPrimitive, Unsigned};

#[cfg(feature = "volatile")]
mod default;
#[cfg(feature = "memmap2")]
mod memmap2;

#[cfg(feature = "volatile")]
pub use default::*;
#[cfg(feature = "memmap2")]
pub use memmap2::*;

pub(super) trait ModelRefAccess<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Get model's length (item count) as `&`
    fn l(&self) -> &K;
    /// Get model's sparse array as `&`
    fn s(&self) -> &[K];
    /// Get model's tight dense array as `&`
    fn d(&self) -> &[K];
}

pub(super) trait ModelMutAccess<K>
where
    K: Unsigned + AsPrimitive<usize> + Copy + PartialOrd,
{
    /// Get model's length (item count) as `&mut`
    fn l(&mut self) -> &mut K;
    /// Get model's sparse array as `&mut`
    fn s(&mut self) -> &mut [K];
    /// Get model's tight dense array as `&mut`
    fn d(&mut self) -> &mut [K];
}
