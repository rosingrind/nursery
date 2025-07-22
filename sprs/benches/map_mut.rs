use std::hint::black_box;

use bencher::Bencher;
use sprs::{
    Key,
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
        let mut map = SparMap::<Key, &str>::new(black_box(Key::MAX as usize));
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
        let mut map = SparMap::<Key, &str>::new(black_box(Key::MAX as usize));
        map.insert_all(add.clone());
        map.delete_all(VEC);
        map.delete_all(VEC);
    });
}

fn retain_all(b: &mut Bencher) {
    let tmp = black_box(VEC)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(Key::MAX as usize));
        map.insert_all(add.clone());
        map.retain(|_, _| black_box(false));
        map.retain(|_, _| black_box(false));
    });
}

fn recall_all(b: &mut Bencher) {
    let tmp = black_box(VEC)
        .map(|x| (x, x.to_string()))
        .collect::<Box<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    b.iter(|| {
        let mut map = SparMap::<Key, &str>::new(black_box(Key::MAX as usize));
        map.insert_all(add.clone());
        black_box(map.recall(|_, _| black_box(true)).collect::<Box<[_]>>());
        black_box(map.recall(|_, _| black_box(true)).collect::<Box<[_]>>());
    });
}

bencher::benchmark_group!(
    benches, insert_one, insert_all, delete_one, delete_all, retain_all, recall_all
);
bencher::benchmark_main!(benches);
