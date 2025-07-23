mod area;

use std::sync::OnceLock;

use itertools::Itertools;
#[cfg(feature = "rayon")]
use rayon::prelude::*;

use crate::{Area, Placement, Rect};

#[derive(Debug, Clone, PartialEq)]
pub struct RectGroup {
    pub list: Box<[Placement<Rect>]>,
    a: OnceLock<u64>,
    f: OnceLock<u64>,
    w: OnceLock<u32>,
    h: OnceLock<u32>,
}

impl RectGroup {
    pub fn score(&self, space: &Placement<Rect>, avg_high: f64) -> u64 {
        space.area() - self.area() + avg_high.round() as u64
    }

    pub fn combine(self, other: Self) -> [Self; 2] {
        let w = self.w();
        let h = self.h();
        [
            self.list
                .clone()
                .into_iter()
                .chain(other.list.clone().into_iter().update(|p| p.x += w))
                .collect(),
            self.list
                .into_iter()
                .chain(other.list.into_iter().update(|p| p.y += h))
                .collect(),
        ]
    }
}

impl FromIterator<Placement<Rect>> for RectGroup {
    fn from_iter<T: IntoIterator<Item = Placement<Rect>>>(iter: T) -> Self {
        Self {
            list: iter.into_iter().collect(),
            a: OnceLock::new(),
            f: OnceLock::new(),
            w: OnceLock::new(),
            h: OnceLock::new(),
        }
    }
}
#[cfg(feature = "rayon")]
impl FromParallelIterator<Placement<Rect>> for RectGroup {
    fn from_par_iter<I: IntoParallelIterator<Item = Placement<Rect>>>(par_iter: I) -> Self {
        Self {
            list: par_iter.into_par_iter().collect(),
            a: OnceLock::new(),
            f: OnceLock::new(),
            w: OnceLock::new(),
            h: OnceLock::new(),
        }
    }
}
