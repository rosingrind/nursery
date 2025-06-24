use std::hint::black_box;

use bencher::Bencher;
use sparsey::World;

fn insert_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<&str>().build());
        black_box(map.create(black_box(("5",))));
        black_box(map.create(black_box(("5",))));
    });
}

fn insert_all(b: &mut Bencher) {
    let tmp = black_box(0..sprs::KEY_MAX)
        .map(|x| (x.to_string().into_boxed_str(),))
        .collect::<Box<[_]>>();

    b.iter(|| {
        let mut map = black_box(World::builder().register::<Box<str>>().build());
        black_box(map.extend(tmp.clone()));
    });
}

fn delete_one(b: &mut Bencher) {
    b.iter(|| {
        let mut map = black_box(World::builder().register::<&str>().build());
        let key = map.create(black_box(("5",)));
        black_box(map.remove::<(&str,)>(key));
        black_box(map.remove::<(&str,)>(key));
    });
}

fn delete_all(b: &mut Bencher) {
    let tmp = black_box(0..sprs::KEY_MAX)
        .map(|x| x.to_string())
        .collect::<Box<[_]>>();

    b.iter(|| {
        let mut map = black_box(World::builder().register::<Box<str>>().build());
        let del = black_box(
            tmp.iter()
                .cloned()
                .map(|i| map.create(black_box((i.into_boxed_str(),)))),
        )
        .collect::<Box<[_]>>();
        for key in del.iter() {
            black_box(map.remove::<(Box<str>,)>(*key));
            black_box(map.remove::<(Box<str>,)>(*key));
        }
    });
}

bencher::benchmark_group!(benches, insert_one, insert_all, delete_one, delete_all);
bencher::benchmark_main!(benches);
