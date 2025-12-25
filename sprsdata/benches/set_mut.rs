use std::hint::black_box;

use bencher::Bencher;
use sprsdata::{Key, SetMut, SparSet};

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
        let mut set = SparSet::<Key>::new(black_box(Key::MAX as usize));
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
        let mut set = SparSet::<Key>::new(black_box(Key::MAX as usize));
        set.insert_all(VEC);
        set.delete_all(VEC);
        set.delete_all(VEC);
    });
}

fn retain_all(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(Key::MAX as usize));
        set.insert_all(VEC);
        set.retain(|_| black_box(false));
        set.retain(|_| black_box(false));
    });
}

fn recall_all(b: &mut Bencher) {
    b.iter(|| {
        let mut set = SparSet::<Key>::new(black_box(Key::MAX as usize));
        set.insert_all(VEC);
        black_box(set.recall(|_| black_box(true)).collect::<Box<[_]>>());
        black_box(set.recall(|_| black_box(true)).collect::<Box<[_]>>());
    });
}

bencher::benchmark_group!(
    benches, insert_one, insert_all, delete_one, delete_all, retain_all, recall_all
);
bencher::benchmark_main!(benches);
