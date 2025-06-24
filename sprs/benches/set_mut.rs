use divan::{Bencher, black_box};
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench()]
fn insert_one() {
    let mut set = black_box(SparSet::<Key>::new(0));
    black_box(&mut set).insert_one(black_box(0));
    black_box(&mut set).insert_one(black_box(0));
}

#[divan::bench()]
fn insert_all(bencher: Bencher) {
    let vec = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let set = SparSet::<Key>::new(KEY_MAX);

    bencher.bench(|| {
        let mut set = black_box(set.clone());
        black_box(&mut set).insert_all(black_box(&vec));
        black_box(&mut set).insert_all(black_box(&vec));
    });
}

#[divan::bench()]
fn delete_one(bencher: Bencher) {
    let mut set = SparSet::<Key>::new(0);
    set.insert_one(black_box(0));

    bencher.bench(|| {
        let mut set = black_box(set.clone());
        black_box(&mut set).delete_one(black_box(0));
        black_box(&mut set).delete_one(black_box(5));
    });
}

#[divan::bench()]
fn delete_all(bencher: Bencher) {
    let vec = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let mut set = SparSet::<Key>::new(KEY_MAX);
    set.insert_all(&vec);

    bencher.bench(|| {
        let mut set = black_box(set.clone());
        black_box(&mut set).delete_all(black_box(&vec));
        black_box(&mut set).delete_all(black_box(&vec));
    });
}

fn main() {
    divan::main();
}
