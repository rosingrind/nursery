use crate::{Area, BspaNode};

impl Area for BspaNode {
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

impl Area for &BspaNode {
    #[inline]
    fn area(&self) -> u64 {
        self.w() as u64 * self.h() as u64
    }

    #[inline]
    fn fill_area(&self) -> u64 {
        self.blocks.iter().map(|x| x.item.area()).sum()
    }

    #[inline]
    fn w(&self) -> u32 {
        let xmin = self.blocks.iter().map(|p| p.x).min().unwrap();
        let xmax = self.blocks.iter().map(|p| p.x + p.w()).max().unwrap();
        xmax - xmin
    }

    #[inline]
    fn h(&self) -> u32 {
        let ymin = self.blocks.iter().map(|p| p.y).min().unwrap();
        let ymax = self.blocks.iter().map(|p| p.y + p.h()).max().unwrap();
        ymax - ymin
    }
}
