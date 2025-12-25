mod ext;
mod map;
mod set;

pub use map::*;
pub use set::*;

pub type Key = u16; // u32 is 17GB

#[cfg(feature = "memmap2")]
/// 4GB
const MAX_BYTE_PRE_LOAD_SIZE: usize = 4usize.pow(30);
