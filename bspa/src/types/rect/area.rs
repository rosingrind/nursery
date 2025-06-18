use crate::{Area, Rect};

impl Area for Rect {
    #[inline]
    fn area(&self) -> u64 {
        Area::area(&self)
    }

    #[inline]
    fn fill_area(&self) -> u64 {
        Area::fill_area(&self)
    }

    #[inline]
    fn w(&self) -> u32 {
        Area::w(&self)
    }

    #[inline]
    fn h(&self) -> u32 {
        Area::h(&self)
    }
}

impl Area for &Rect {
    #[inline]
    fn area(&self) -> u64 {
        self.s
    }

    #[inline]
    fn fill_area(&self) -> u64 {
        self.s
    }

    #[inline]
    fn w(&self) -> u32 {
        self.w
    }

    #[inline]
    fn h(&self) -> u32 {
        self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RECT_S: Rect = Rect::new(8, 8);
    const RECT_L: Rect = Rect::new(16, 16);

    #[test]
    fn area() {
        assert_eq!(RECT_S.area(), 64);
        assert_eq!(RECT_S.fill_area(), 64);
    }
}
