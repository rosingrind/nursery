use std::hint::black_box;

use bencher::Bencher;
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn contains(b: &mut Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(black_box(0), black_box("0"));

    b.iter(|| {
        let map = black_box(map.clone());
        map.contains(black_box(0));
        map.contains(black_box(5));
    });
}

fn query_one(b: &mut Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(black_box(0), black_box("0"));

    b.iter(|| {
        let map = black_box(map.clone());
        map.query_one(black_box(0));
        map.query_one(black_box(5));
    });
}

fn query_all(b: &mut Bencher) {
    let mut map = SparMap::<Key, &str>::new(KEY_MAX);
    let vec = black_box(0..Key::MAX).collect_array::<KEY_MAX>().unwrap();
    let tmp = black_box(0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();
    map.insert_all(add);

    b.iter(|| {
        let map = black_box(map.clone());
        black_box(map.query_all(black_box(&vec)).count());
        black_box(map.query_all(black_box(&vec)).count());
    });
}

bencher::benchmark_group!(benches, contains, query_one, query_all);
bencher::benchmark_main!(benches);
