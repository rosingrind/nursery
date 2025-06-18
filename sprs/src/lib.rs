pub mod map;
pub mod set;

pub type Key = u16; // u32 is 17GB
pub const KEY_MAX: usize = Key::MAX as usize;
