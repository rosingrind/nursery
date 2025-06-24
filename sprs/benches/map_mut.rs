use divan::{Bencher, black_box};
use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, SparMap},
};

#[divan::bench()]
fn insert_one() {
    let mut map = SparMap::<Key, &str>::new(0);
    black_box(&mut map).insert_one(black_box(0), black_box("0"));
    black_box(&mut map).insert_one(black_box(0), black_box("0"));
}

#[divan::bench()]
fn insert_all(bencher: Bencher) {
    let map = SparMap::<Key, &str>::new(KEY_MAX);
    let tmp = (0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Vec<_>>();
    let vec = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).insert_all(black_box(vec.clone()));
        black_box(&mut map).insert_all(black_box(vec.clone()));
    });
}

#[divan::bench()]
fn delete_one(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new(0);
    map.insert_one(0, "0");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).delete_one(black_box(0));
        black_box(&mut map).delete_one(black_box(5));
    });
}

#[divan::bench()]
fn delete_all(bencher: Bencher) {
    let mut map = SparMap::<Key, &str>::new(KEY_MAX);
    let tmp = (0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Vec<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();
    map.insert_all(add);
    let del = (0..Key::MAX).collect_array::<KEY_MAX>().unwrap();

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).delete_all(black_box(&del));
        black_box(&mut map).delete_all(black_box(&del));
    });
}

fn main() {
    divan::main();
}
