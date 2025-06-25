use std::hint::black_box;

use bencher::Bencher;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SparSet},
};

const VEC: std::ops::Range<u16> = 0..Key::MAX;

fn insert_one(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(0));
        black_box(set.insert_one(black_box(0)));
        black_box(set.insert_one(black_box(0)));
    });
}

fn insert_all(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(KEY_MAX));
        set.insert_all(VEC);
        set.insert_all(VEC);
    });
}

fn delete_one(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(0));
        set.insert_one(black_box(0));
        black_box(set.delete_one(black_box(0)));
        black_box(set.delete_one(black_box(5)));
    });
}

fn delete_all(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(KEY_MAX));
        set.insert_all(VEC);
        set.delete_all(VEC);
        set.delete_all(VEC);
    });
}

bencher::benchmark_group!(benches, insert_one, insert_all, delete_one, delete_all);
bencher::benchmark_main!(benches);
