use divan::{Bencher, black_box};
use sparsey::World;
use sprs::Key;

#[divan::bench()]
fn insert_one(bencher: Bencher) {
    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<&str>().build());
        black_box(&mut map).create(black_box(("5",)));
        black_box(&mut map).create(black_box(("5",)));
    });
}

#[divan::bench()]
fn insert_all(bencher: Bencher) {
    let tmp = (0..Key::MAX)
        .map(|x| (x.to_string().into_boxed_str(),))
        .collect::<Vec<_>>();

    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<Box<str>>().build());
        black_box(black_box(&mut map).extend(tmp.clone().into_iter()));
    });
}

#[divan::bench()]
fn delete_one(bencher: Bencher) {
    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<&str>().build());
        let key = black_box(&mut map).create(black_box(("5",)));
        black_box(map.remove::<(&str,)>(key));
        black_box(map.remove::<(&str,)>(key));
    });
}

#[divan::bench()]
fn delete_all(bencher: Bencher) {
    let tmp = (0..Key::MAX).map(|x| x.to_string()).collect::<Vec<_>>();

    bencher.bench(|| {
        let mut map = black_box(World::builder().register::<Box<str>>().build());
        let del = black_box(
            tmp.iter()
                .cloned()
                .map(|i| black_box(&mut map).create(black_box((i.into_boxed_str(),)))),
        )
        .collect::<Vec<_>>();
        for key in del.iter() {
            black_box(map.remove::<(Box<str>,)>(*key));
            black_box(map.remove::<(Box<str>,)>(*key));
        }
    });
}

fn main() {
    divan::main();
}
