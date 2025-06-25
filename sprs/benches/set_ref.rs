use std::hint::black_box;

use bencher::Bencher;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const VEC: std::ops::Range<u16> = 0..Key::MAX;

fn contains(b: &mut Bencher) {
    b.iter(move || {
        let mut set = SparSet::<Key>::new(black_box(0));
        set.insert_one(black_box(0));
        black_box(set.contains(black_box(0)));
        black_box(set.contains(black_box(5)));
    });
}

fn intersection(b: &mut Bencher) {
    b.iter(|| {
        let mut l = SparSet::<Key>::new(black_box(KEY_MAX));
        let mut r = SparSet::<Key>::new(black_box(KEY_MAX));
        l.insert_all(VEC);
        r.insert_all(VEC.rev());

        black_box(l.intersection(&r).collect::<Box<_>>());
        black_box(r.intersection(&l).collect::<Box<_>>());
    });
}

fn union(b: &mut Bencher) {
    b.iter(|| {
        let mut l = SparSet::<Key>::new(black_box(KEY_MAX));
        let mut r = SparSet::<Key>::new(black_box(KEY_MAX));
        l.insert_all(VEC);
        r.insert_all(VEC.rev());

        black_box(l.union(&r).collect::<Box<_>>());
        black_box(r.union(&l).collect::<Box<_>>());
    });
}

fn difference(b: &mut Bencher) {
    b.iter(|| {
        let mut l = SparSet::<Key>::new(black_box(KEY_MAX));
        let mut r = SparSet::<Key>::new(black_box(KEY_MAX));
        l.insert_all(VEC);
        r.insert_all(VEC.rev());

        black_box(l.difference(&r).collect::<Box<_>>());
        black_box(r.difference(&l).collect::<Box<_>>());
    });
}

bencher::benchmark_group!(benches, contains, intersection, union, difference);
bencher::benchmark_main!(benches);
