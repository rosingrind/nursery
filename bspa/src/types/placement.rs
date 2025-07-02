mod area;
#[cfg(test)]
mod tests;

use std::hash::Hash;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{Area, Rect, RectGroup};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement<T: Area> {
    pub x: u32,
    pub y: u32,
    pub item: T,
}

impl<T: Area> Placement<T> {
    pub fn overlaps<U: Area>(&self, rhs: &Placement<U>) -> bool {
        let l_x = self.x + self.item.w();
        let l_y = self.y + self.item.h();
        let r_x = rhs.x + rhs.item.w();
        let r_y = rhs.y + rhs.item.h();

        ((rhs.x < l_x) & (r_x > self.x)) & ((rhs.y < l_y) & (r_y > self.y))
    }

    /// Split in `y` positive direction
    pub fn split_n<U: Area>(&self, rhs: &Placement<U>) -> Option<Placement<Rect>> {
        std::hint::select_unpredictable(
            rhs.y > self.y,
            Some(Placement {
                x: self.x,
                y: self.y,
                item: Rect::new(self.item.w(), rhs.y.saturating_sub(self.y)),
            }),
            None,
        )
    }

    /// Split in `y` negative direction
    pub fn split_s<U: Area>(&self, rhs: &Placement<U>) -> Option<Placement<Rect>> {
        std::hint::select_unpredictable(
            rhs.y + rhs.item.h() < self.y + self.item.h(),
            Some(Placement {
                x: self.x,
                y: rhs.y + rhs.item.h(),
                item: Rect::new(
                    self.item.w(),
                    (self.y + self.item.h()).saturating_sub(rhs.y + rhs.item.h()),
                ),
            }),
            None,
        )
    }

    /// Split in `x` positive direction
    pub fn split_e<U: Area>(&self, rhs: &Placement<U>) -> Option<Placement<Rect>> {
        std::hint::select_unpredictable(
            rhs.x > self.x,
            Some(Placement {
                x: self.x,
                y: self.y,
                item: Rect::new(rhs.x.saturating_sub(self.x), self.item.h()),
            }),
            None,
        )
    }

    /// Split in `x` negative direction
    pub fn split_w<U: Area>(&self, rhs: &Placement<U>) -> Option<Placement<Rect>> {
        std::hint::select_unpredictable(
            rhs.x + rhs.item.w() < self.x + self.item.w(),
            Some(Placement {
                x: rhs.x + rhs.item.w(),
                y: self.y,
                item: Rect::new(
                    (self.x + self.item.w()).saturating_sub(rhs.x + rhs.item.w()),
                    self.item.h(),
                ),
            }),
            None,
        )
    }

    #[allow(dead_code)]
    pub fn substract<U: Area>(self, rhs: &Placement<U>) -> impl Iterator<Item = Placement<Rect>>
    where
        T: Copy,
    {
        [
            self.split_n(rhs),
            self.split_s(rhs),
            self.split_e(rhs),
            self.split_w(rhs),
        ]
        .into_iter()
        .filter_map(|x| x.filter(|c| c.item.area() > 0))
    }

    #[cfg(feature = "rayon")]
    pub fn par_substract<U: Area + Sync>(
        self,
        rhs: &Placement<U>,
    ) -> impl ParallelIterator<Item = Placement<Rect>>
    where
        T: Sync,
    {
        [
            self.split_n(rhs),
            self.split_s(rhs),
            self.split_e(rhs),
            self.split_w(rhs),
        ]
        .into_par_iter()
        .filter_map(|x| x.filter(|c| c.item.area() > 0))
    }
}

impl Placement<RectGroup> {
    pub fn into_placed_rects(self) -> impl Iterator<Item = Placement<Rect>> {
        self.item.list.into_iter().map(move |p| Placement {
            x: self.x + p.x,
            y: self.y + p.y,
            item: p.item,
        })
    }
}

impl<T: Area + PartialEq + Eq> PartialOrd for Placement<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Area + Eq> Ord for Placement<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min `y` then min `x` order
        ((self.x as u64) | ((self.y as u64) << 8))
            .cmp(&((other.x as u64) | ((other.y as u64) << 8)))
    }
}

impl<T: Area + Hash> Hash for Placement<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.item.hash(state);
        state.write_u64((self.x as u64) | ((self.y as u64) << 8));
    }
}
