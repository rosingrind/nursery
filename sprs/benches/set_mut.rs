use divan::{Bencher, black_box};
use sprs::{
    Key,
    set::{SetMut, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench(threads = [0, 1])]
fn insert_one(bencher: Bencher) {
    bencher.bench(|| {
        let mut set = black_box(SparSet::<Key>::new());
        black_box(&mut set).insert_one(black_box(5));
        black_box(&mut set).insert_one(black_box(5));
    });
}

#[divan::bench(threads = [0, 1])]
fn insert_all(bencher: Bencher) {
    let vec = (0..Key::MAX).collect::<Vec<_>>();

    bencher.bench(|| {
        let mut set = black_box(SparSet::new());
        black_box(&mut set).insert_all(black_box(vec.clone()));
        black_box(&mut set).insert_all(black_box(vec.clone()));
    });
}

#[divan::bench(threads = [0, 1])]
fn delete_one(bencher: Bencher) {
    bencher.bench(|| {
        let mut set = black_box(SparSet::<Key>::new());
        black_box(&mut set).insert_one(black_box(5));
        black_box(&mut set).delete_one(black_box(5));
        black_box(&mut set).delete_one(black_box(5));
    });
}

#[divan::bench(threads = [0, 1])]
fn delete_all(bencher: Bencher) {
    let vec = (0..Key::MAX).collect::<Vec<_>>();

    bencher.bench(|| {
        let mut set = black_box(SparSet::new());
        black_box(&mut set).insert_all(vec.clone());
        black_box(&mut set).delete_all(black_box(vec.clone()));
    });
}

fn main() {
    divan::main();
}
