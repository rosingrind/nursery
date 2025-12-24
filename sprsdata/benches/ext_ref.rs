use std::hint::black_box;

use bencher::Bencher;
use sparsey::World;

#[derive(Debug, PartialEq, Eq)]
struct Data(String);

fn contains(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = map.create(black_box((Data("5".to_string()),)));
        black_box(map.query_one::<&Data>().get(key).unwrap());
    });
}

fn query_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        let key = map.create(black_box((Data("5".to_string()),)));
        assert_eq!(
            map.query_one::<&Data>().get(key).unwrap(),
            black_box(&Data("5".to_string()))
        );
    });
}

fn query_all(b: &mut Bencher) {
    let tmp = black_box(0..sprs::Key::MAX as usize)
        .map(|x| x.to_string())
        .collect::<Box<[_]>>();

    b.iter(|| {
        let mut map = black_box(World::builder().register::<Data>().build());
        for i in tmp.iter().cloned() {
            map.create(black_box((Data(i),)));
        }
        black_box(map.query_all::<&Data>().into_iter().collect::<Box<[_]>>());
    });
}

bencher::benchmark_group!(benches, contains, query_one, query_all);
bencher::benchmark_main!(benches);
