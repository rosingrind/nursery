mod ext;
pub mod map;
pub mod set;

pub type Key = u16; // u32 is 17GB

#[cfg(feature = "memmap2")]
/// 4GB
const MAX_BYTE_PRE_LOAD_SIZE: usize = 4usize.pow(30);
