use std::hint::black_box;

use bencher::Bencher;
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn insert_one(b: &mut Bencher) {
    b.iter(|| {
        let mut set = black_box(SparSet::<Key>::new(0));
        set.insert_one(black_box(0));
        set.insert_one(black_box(0));
    });
}

fn insert_all(b: &mut Bencher) {
    let vec = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let set = SparSet::<Key>::new(KEY_MAX);

    b.iter(|| {
        let mut set = black_box(set.clone());
        set.insert_all(black_box(vec));
        set.insert_all(black_box(vec));
    });
}

fn delete_one(b: &mut Bencher) {
    let mut set = SparSet::<Key>::new(0);
    set.insert_one(black_box(0));

    b.iter(|| {
        let mut set = black_box(set.clone());
        set.delete_one(black_box(0));
        set.delete_one(black_box(5));
    });
}

fn delete_all(b: &mut Bencher) {
    let vec = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let mut set = SparSet::<Key>::new(KEY_MAX);
    set.insert_all(vec);

    b.iter(|| {
        let mut set = black_box(set.clone());
        set.delete_all(black_box(vec));
        set.delete_all(black_box(vec));
    });
}

bencher::benchmark_group!(benches, insert_one, insert_all, delete_one, delete_all);
bencher::benchmark_main!(benches);
