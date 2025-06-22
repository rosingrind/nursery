use divan::{Bencher, black_box};
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench()]
fn contains(bencher: Bencher) {
    let mut set = SparSet::<Key>::new(KEY_MAX);
    set.insert_one(5);

    bencher.bench(move || {
        let mut set = black_box(set.clone());
        black_box(&mut set).contains(black_box(5));
        black_box(&mut set).contains(black_box(0));
    });
}

#[divan::bench()]
fn intersection(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_b = (0..Key::MAX).rev().collect_array::<KEY_MAX>().unwrap();
    let a = SparSet::<Key>::new(KEY_MAX);
    let b = SparSet::<Key>::new(KEY_MAX);

    bencher.bench(|| {
        let mut a = black_box(a.clone());
        let mut b = black_box(b.clone());
        black_box(&mut a).insert_all(black_box(&vec_a));
        black_box(&mut b).insert_all(black_box(&vec_b));

        black_box(a.intersection(&b).collect::<Vec<_>>());
        black_box(b.intersection(&a).collect::<Vec<_>>());
    });
}

#[divan::bench()]
fn union(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_b = (0..Key::MAX).rev().collect_array::<KEY_MAX>().unwrap();
    let a = SparSet::<Key>::new(KEY_MAX);
    let b = SparSet::<Key>::new(KEY_MAX);

    bencher.bench(|| {
        let mut a = black_box(a.clone());
        let mut b = black_box(b.clone());
        black_box(&mut a).insert_all(black_box(&vec_a));
        black_box(&mut b).insert_all(black_box(&vec_b));

        black_box(a.union(&b).collect::<Vec<_>>());
        black_box(b.union(&a).collect::<Vec<_>>());
    });
}

#[divan::bench()]
fn difference(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let vec_b = (0..Key::MAX).rev().collect_array::<KEY_MAX>().unwrap();
    let a = SparSet::<Key>::new(KEY_MAX);
    let b = SparSet::<Key>::new(KEY_MAX);

    bencher.bench(|| {
        let mut a = black_box(a.clone());
        let mut b = black_box(b.clone());
        black_box(&mut a).insert_all(black_box(&vec_a));
        black_box(&mut b).insert_all(black_box(&vec_b));

        black_box(a.difference(&b).collect::<Vec<_>>());
        black_box(b.difference(&a).collect::<Vec<_>>());
    });
}

fn main() {
    divan::main();
}
