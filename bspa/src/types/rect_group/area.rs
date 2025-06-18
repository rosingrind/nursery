use crate::{Area, RectGroup};

impl Area for RectGroup {
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

impl Area for &RectGroup {
    #[inline]
    fn area(&self) -> u64 {
        *self.a.get_or_init(|| self.w() as u64 * self.h() as u64)
    }

    #[inline]
    fn fill_area(&self) -> u64 {
        *self
            .f
            .get_or_init(|| self.list.iter().map(|x| x.item.area()).sum())
    }

    #[inline]
    fn w(&self) -> u32 {
        *self.w.get_or_init(|| {
            let xmin = self.list.iter().map(|p| p.x).min().unwrap();
            let xmax = self.list.iter().map(|p| p.x + p.w()).max().unwrap();
            xmax - xmin
        })
    }

    #[inline]
    fn h(&self) -> u32 {
        *self.h.get_or_init(|| {
            let ymin = self.list.iter().map(|p| p.y).min().unwrap();
            let ymax = self.list.iter().map(|p| p.y + p.h()).max().unwrap();
            ymax - ymin
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Placement, Rect};

    use super::*;

    const RECT_S: Rect = Rect::new(8, 8);
    const RECT_L: Rect = Rect::new(16, 16);

    #[test]
    fn area() {
        let rg = [
            Placement {
                x: 0,
                y: 0,
                item: RECT_L,
            },
            Placement {
                x: 16,
                y: 0,
                item: RECT_S,
            },
            Placement {
                x: 16,
                y: 8,
                item: RECT_S,
            },
            Placement {
                x: 24,
                y: 0,
                item: RECT_S,
            },
            Placement {
                x: 24,
                y: 8,
                item: RECT_S,
            },
        ]
        .into_iter()
        .collect::<RectGroup>();

        assert_eq!(rg.area(), 512);
        assert_eq!(rg.fill_area(), 512);

        let rg = [
            Placement {
                x: 0,
                y: 0,
                item: RECT_S,
            },
            Placement {
                x: 0,
                y: 8,
                item: RECT_S,
            },
            Placement {
                x: 0,
                y: 16,
                item: RECT_S,
            },
            Placement {
                x: 0,
                y: 24,
                item: RECT_S,
            },
            Placement {
                x: 0,
                y: 32,
                item: RECT_S,
            },
        ]
        .into_iter()
        .collect::<RectGroup>();

        assert_eq!(rg.area(), 320);
        assert_eq!(rg.fill_area(), 320);
    }
}
