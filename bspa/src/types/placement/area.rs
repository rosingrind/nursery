use crate::{Area, Placement};

impl<T: Area> Area for Placement<T> {
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

impl<T: Area> Area for &Placement<T> {
    #[inline]
    fn area(&self) -> u64 {
        self.item.area()
    }

    #[inline]
    fn fill_area(&self) -> u64 {
        self.item.fill_area()
    }

    #[inline]
    fn w(&self) -> u32 {
        self.item.w()
    }

    #[inline]
    fn h(&self) -> u32 {
        self.item.h()
    }
}
