use divan::{Bencher, black_box};
use sparsey::World;
use sprs::Key;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(PartialEq, Eq)]
struct Data(String);

#[divan::bench()]
fn contains(bencher: Bencher) {
    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = black_box(&mut map).create(black_box((Data("5".to_string()),)));
        black_box(
            black_box(black_box(&mut map).query_one::<&Data>().get(key).unwrap())
                == &Data("5".to_string()),
        );
    });
}

#[divan::bench()]
fn query_one(bencher: Bencher) {
    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = black_box(&mut map).create(black_box((Data("5".to_string()),)));
        black_box(black_box(&mut map).query_one::<&Data>().get(key).unwrap());
    });
}

#[divan::bench()]
fn query_all(bencher: Bencher) {
    let tmp = (0..Key::MAX).map(|x| x.to_string()).collect::<Vec<_>>();

    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        for i in tmp.iter().cloned() {
            black_box(&mut map).create(black_box((Data(i),)));
        }
        black_box(
            black_box(&mut map)
                .query_all::<&Data>()
                .into_iter()
                .collect::<Vec<_>>(),
        );
    });
}

fn main() {
    divan::main();
}
