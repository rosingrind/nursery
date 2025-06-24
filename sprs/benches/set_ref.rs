use std::hint::black_box;

use bencher::Bencher;
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn contains(b: &mut Bencher) {
    let mut set = SparSet::<Key>::new(0);
    set.insert_one(black_box(0));

    b.iter(move || {
        let set = black_box(set.clone());
        set.contains(black_box(0));
        set.contains(black_box(5));
    });
}

fn intersection(b: &mut Bencher) {
    let vec_l = black_box(0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_r = black_box(0..Key::MAX)
        .rev()
        .collect_array::<KEY_MAX>()
        .unwrap();
    let l = SparSet::<Key>::new(KEY_MAX);
    let r = SparSet::<Key>::new(KEY_MAX);

    b.iter(|| {
        let mut l = black_box(l.clone());
        let mut r = black_box(r.clone());
        l.insert_all(black_box(vec_l));
        r.insert_all(black_box(vec_r));

        black_box(l.intersection(&r).collect::<Box<_>>());
        black_box(r.intersection(&l).collect::<Box<_>>());
    });
}

fn union(b: &mut Bencher) {
    let vec_l = black_box(0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_r = black_box(0..Key::MAX)
        .rev()
        .collect_array::<KEY_MAX>()
        .unwrap();
    let l = SparSet::<Key>::new(KEY_MAX);
    let r = SparSet::<Key>::new(KEY_MAX);

    b.iter(|| {
        let mut l = black_box(l.clone());
        let mut r = black_box(r.clone());
        l.insert_all(black_box(vec_l));
        r.insert_all(black_box(vec_r));

        black_box(l.union(&r).collect::<Box<_>>());
        black_box(r.union(&l).collect::<Box<_>>());
    });
}

fn difference(b: &mut Bencher) {
    let vec_l = black_box(0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_r = black_box(0..Key::MAX)
        .rev()
        .collect_array::<KEY_MAX>()
        .unwrap();
    let l = SparSet::<Key>::new(KEY_MAX);
    let r = SparSet::<Key>::new(KEY_MAX);

    b.iter(|| {
        let mut l = black_box(l.clone());
        let mut r = black_box(r.clone());
        l.insert_all(black_box(vec_l));
        r.insert_all(black_box(vec_r));

        black_box(l.difference(&r).collect::<Box<_>>());
        black_box(r.difference(&l).collect::<Box<_>>());
    });
}

bencher::benchmark_group!(benches, contains, intersection, union, difference);
bencher::benchmark_main!(benches);
