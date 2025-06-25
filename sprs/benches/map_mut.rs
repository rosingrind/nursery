use std::hint::black_box;

use bencher::Bencher;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

const VEC: std::ops::Range<u16> = 0..Key::MAX;

fn insert_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(0));
        black_box(map.insert_one(black_box(0), black_box("0")));
        black_box(map.insert_one(black_box(0), black_box("0")));
    });
}

fn insert_all(b: &mut Bencher) {
    let tmp = black_box(VEC)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let vec = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(KEY_MAX));
        map.insert_all(vec.clone());
        map.insert_all(vec.clone());
    });
}

fn delete_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(0));
        map.insert_one(black_box(0), black_box("0"));
        black_box(map.delete_one(black_box(0)));
        black_box(map.delete_one(black_box(5)));
    });
}

fn delete_all(b: &mut Bencher) {
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
        map.delete_all(VEC);
        map.delete_all(VEC);
    });
}

bencher::benchmark_group!(benches, insert_one, insert_all, delete_one, delete_all);
bencher::benchmark_main!(benches);
