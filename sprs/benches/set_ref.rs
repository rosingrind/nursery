use divan::{Bencher, black_box};
use sprs::{
    Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench(threads = [0, 1])]
fn contains(bencher: Bencher) {
    bencher.bench(move || {
        let mut set = black_box(SparSet::<Key>::new());
        black_box(&mut set).insert_one(black_box(5));
        black_box(&mut set).contains(black_box(5));
        black_box(&mut set).contains(black_box(0));
    });
}

#[divan::bench(threads = [0, 1])]
fn intersection(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect::<Vec<_>>();
    let vec_b = (0..Key::MAX).rev().collect::<Vec<_>>();

    bencher.bench(|| {
        let mut a = black_box(SparSet::new());
        let mut b = black_box(SparSet::new());
        black_box(&mut a).insert_all(black_box(vec_a.clone()));
        black_box(&mut b).insert_all(black_box(vec_b.clone()));

        black_box(a.intersection(&b).collect::<Vec<_>>());
        black_box(b.intersection(&a).collect::<Vec<_>>());
    });
}

#[divan::bench(threads = [0, 1])]
fn union(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect::<Vec<_>>();
    let vec_b = (0..Key::MAX).rev().collect::<Vec<_>>();

    bencher.bench(|| {
        let mut a = black_box(SparSet::new());
        let mut b = black_box(SparSet::new());
        black_box(&mut a).insert_all(black_box(vec_a.clone()));
        black_box(&mut b).insert_all(black_box(vec_b.clone()));

        black_box(a.union(&b).collect::<Vec<_>>());
        black_box(b.union(&a).collect::<Vec<_>>());
    });
}

#[divan::bench(threads = [0, 1])]
fn difference(bencher: Bencher) {
    let vec_a = (0..Key::MAX).collect::<Vec<_>>();
    let vec_b = (0..Key::MAX).rev().collect::<Vec<_>>();

    bencher.bench(|| {
        let mut a = black_box(SparSet::new());
        let mut b = black_box(SparSet::new());
        black_box(&mut a).insert_all(black_box(vec_a.clone()));
        black_box(&mut b).insert_all(black_box(vec_b.clone()));

        black_box(a.difference(&b).collect::<Vec<_>>());
        black_box(b.difference(&a).collect::<Vec<_>>());
    });
}

fn main() {
    divan::main();
}
