use divan::{Bencher, black_box};
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench()]
fn contains(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(0, "0");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).contains(black_box(0));
        black_box(&mut map).contains(black_box(5));
    });
}

#[divan::bench()]
fn query_one(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(0, "0");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).query_one(black_box(0));
        black_box(&mut map).query_one(black_box(5));
    });
}

#[divan::bench()]
fn query_all(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new(KEY_MAX);
    let vec = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let tmp = (0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Vec<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();
    map.insert_all(add);

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(black_box(&mut map).query_all(black_box(&vec)).count());
        black_box(black_box(&mut map).query_all(black_box(&vec)).count());
    });
}

fn main() {
    divan::main();
}
