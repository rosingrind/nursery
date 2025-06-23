#[cfg(feature = "rayon")]
mod par;
#[cfg(not(feature = "rayon"))]
mod seq;

#[cfg(feature = "rayon")]
pub use par::*;
#[cfg(not(feature = "rayon"))]
pub use seq::*;
