use std::collections::{HashMap, HashSet};

use bspa::{Area, Placement, Rect, RectGroup};
use image::{ImageBuffer, Pixel, Rgb};
use rand::prelude::*;

fn gen_color<const MIN: u8, const MAX: u8>(rng: &mut ThreadRng) -> Rgb<u8> {
    Rgb([
        rng.random_range(MIN..MAX),
        rng.random_range(MIN..MAX),
        rng.random_range(MIN..MAX),
    ])
}

fn gen_color_map(list: &HashSet<Rect>) -> HashMap<Rect, Rgb<u8>> {
    let mut rng = rand::rng();
    list.iter()
        .map(|&x| (x, gen_color::<64, 255>(&mut rng)))
        .collect::<HashMap<_, _>>()
}

pub fn save_rg(list: &HashSet<Rect>, data: &RectGroup, name: &str) {
    let color_map = gen_color_map(list);

    let w = data.w();
    let h = data.h();
    let mut atlas = ImageBuffer::from_pixel(w, h, Rgb([0u8, 0u8, 0u8]));

    for bp in data.list.iter() {
        let pixel = *color_map.get(&bp.item).unwrap();

        for x in 1..bp.item.w() - 1 {
            for y in 1..bp.item.h() - 1 {
                atlas.put_pixel(bp.x + x, bp.y + y, pixel);
            }
        }
    }
    atlas.save(name).unwrap();
}

pub fn save_pg(list: &HashSet<Rect>, data: &[Placement<RectGroup>], name: &str) {
    let mut rng = rand::rng();
    let color_map = gen_color_map(list);

    let w = data.iter().map(|p| p.x + p.w()).max().unwrap() * 2;
    let h = data.iter().map(|p| p.y + p.h()).max().unwrap() * 2;
    let mut atlas = ImageBuffer::from_pixel(w, h, Rgb([0u8, 0u8, 0u8]));

    for p in data {
        let tint = gen_color::<64, 192>(&mut rng);
        for r in p.item.list.iter() {
            let mut pixel = *color_map
                .get(&r.item)
                .unwrap_or_else(|| panic!("{:?}", r.item));
            pixel.blend(&tint);
            pixel.apply(|p| p.saturating_add(16));

            for x in 1..(r.item.w() * 2 - 1) {
                for y in 1..(r.item.h() * 2 - 1) {
                    atlas.put_pixel((p.x + r.x) * 2 + x, (p.y + r.y) * 2 + y, pixel);
                }
            }
        }
    }

    atlas.save(name).unwrap();
}
