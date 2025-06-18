use divan::{Bencher, black_box};
use sprs::{
    Key,
    map::{MapMut, SparMap},
};

#[divan::bench(threads = [0, 1])]
fn insert_one(bencher: Bencher) {
    let map = SparMap::<&str>::new();

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).insert_one(black_box(5), black_box("5"));
        black_box(&mut map).insert_one(black_box(5), black_box("5"));
    });
}

#[divan::bench(threads = [0, 1])]
fn insert_all(bencher: Bencher) {
    let map = SparMap::<&str>::new();
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

#[divan::bench(threads = [0, 1])]
fn delete_one(bencher: Bencher) {
    let mut map = SparMap::<&str>::new();
    map.insert_one(5, "5");

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).delete_one(black_box(5));
        black_box(&mut map).delete_one(black_box(5));
    });
}

#[divan::bench(threads = [0, 1])]
fn delete_all(bencher: Bencher) {
    let mut map = SparMap::<&str>::new();
    let tmp = (0..Key::MAX)
        .map(|x| (x, x.to_string()))
        .collect::<Vec<_>>();
    let add = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();
    map.insert_all(add);
    let del = (0..Key::MAX).collect::<Vec<_>>();

    bencher.bench(|| {
        let mut map = black_box(map.clone());
        black_box(&mut map).delete_all(black_box(&del));
        black_box(&mut map).delete_all(black_box(&del));
    });
}

fn main() {
    divan::main();
}
