use divan::{Bencher, black_box};
use sprs::{
    Key,
    map::{MapMut, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[divan::bench(threads = [0, 1])]
fn contains(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new();
    map.insert_one(5, "5");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).contains(black_box(5));
        black_box(&mut map).contains(black_box(0));
    });
}

#[divan::bench(threads = [0, 1])]
fn query_one(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new();
    map.insert_one(5, "5");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).query_one(black_box(5));
        black_box(&mut map).query_one(black_box(5));
    });
}

#[divan::bench(threads = [0, 1])]
fn query_all(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new();
    let vec = (0..Key::MAX).collect::<Vec<_>>();
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
