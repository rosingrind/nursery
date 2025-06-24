use std::hint::black_box;

use bencher::Bencher;
use sparsey::World;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Data(String);

fn contains(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = map.create(black_box((Data("5".to_string()),)));
        assert_eq!(
            map.query_one::<&Data>().get(key).unwrap(),
            black_box(&Data("5".to_string()))
        );
    });
}

fn query_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = map.create(black_box((Data("5".to_string()),)));
        black_box(map.query_one::<&Data>().get(key).unwrap());
    });
}

fn query_all(b: &mut Bencher) {
    let tmp = black_box(0..sprs::KEY_MAX)
        .map(|x| x.to_string())
        .collect::<Vec<_>>();

    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        for i in tmp.iter().cloned() {
            map.create(black_box((Data(i),)));
        }
        black_box(map.query_all::<&Data>().into_iter().collect::<Vec<_>>());
    });
}

bencher::benchmark_group!(benches, contains, query_one, query_all);
bencher::benchmark_main!(benches);
