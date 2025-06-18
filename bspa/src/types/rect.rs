mod area;

use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    w: u32,
    h: u32,
    s: u64,
}

impl Rect {
    pub const fn new(w: u32, h: u32) -> Self {
        Self {
            w,
            h,
            s: w as u64 * h as u64,
        }
    }
}

impl Hash for Rect {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64((self.w as u64) | ((self.h as u64) << 8));
    }
}
