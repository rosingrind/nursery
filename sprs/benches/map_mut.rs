use std::hint::black_box;

use bencher::Bencher;
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

fn insert_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(0);
        map.insert_one(black_box(0), black_box("0"));
        map.insert_one(black_box(0), black_box("0"));
    });
}

fn insert_all(b: &mut Bencher) {
    let map = SparMap::<Key, &str>::new(black_box(KEY_MAX));
    let tmp = black_box(0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let vec = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    b.iter(|| {
        let mut map = map.clone();
        map.insert_all(vec.clone());
        map.insert_all(vec.clone());
    });
}

fn delete_one(b: &mut Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(black_box(0), black_box("0"));

    b.iter(|| {
        let mut map = black_box(map.clone());
        map.delete_one(black_box(0));
        map.delete_one(black_box(5));
    });
}

fn delete_all(b: &mut Bencher) {
    let mut map = SparMap::<Key, &str>::new(KEY_MAX);
    let tmp = black_box(0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();
    map.insert_all(add);
    let del = black_box(0..Key::MAX).collect_array::<KEY_MAX>().unwrap();

    b.iter(|| {
        let mut map = black_box(map.clone());
        map.delete_all(black_box(del));
        map.delete_all(black_box(del));
    });
}

bencher::benchmark_group!(benches, insert_one, insert_all, delete_one, delete_all);
bencher::benchmark_main!(benches);
