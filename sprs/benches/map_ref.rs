use std::hint::black_box;

use bencher::Bencher;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn contains(b: &mut Bencher) {
    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(0));
        map.insert_one(black_box(0), black_box("0"));
        black_box(map.contains(black_box(0)));
        black_box(map.contains(black_box(5)));
    });
}

fn query_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(0));
        map.insert_one(black_box(0), black_box("0"));
        black_box(map.query_one(black_box(0)));
        black_box(map.query_one(black_box(5)));
    });
}

fn query_all(b: &mut Bencher) {
    const VEC: std::ops::Range<u16> = 0..Key::MAX;

    let tmp = black_box(VEC)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(KEY_MAX));
        map.insert_all(add.clone());
        black_box(map.query_all(VEC).collect::<Box<_>>());
        black_box(map.query_all(VEC).collect::<Box<_>>());
    });
}

bencher::benchmark_group!(benches, contains, query_one, query_all);
bencher::benchmark_main!(benches);
