use sprs::{Key, map::*};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const fn expand(x: Key) -> (u16, u64) {
    (x, x as u64)
}
const VEC: std::ops::Range<Key> = 0..Key::MAX;
const KEY: Key = VEC.end - 1;

#[test]
fn insert_all() {
    #[cfg(feature = "volatile")]
    let mut map = SparMap::<_, _>::new(Key::MAX as usize);

    #[cfg(feature = "memmap2")]
    let mut map = {
        use std::fs::File;

        const PATH: &str = "map_uge.insert_all.bin";

        let file = File::create_new(PATH)
            .and_then(|f| {
                f.set_len(SparMap::<Key, u64>::file_size(Key::MAX as usize))?;
                Ok(f)
            })
            .or(File::options().read(true).write(true).open(PATH))
            .unwrap();
        SparMap::<_, _>::from_buf(Key::MAX as usize, file)
    };

    map.insert_all(VEC.map(expand));
    itertools::assert_equal(map.iter().map(|(k, v)| (*k, *v)), VEC.map(expand));
    map.insert_all(VEC.map(expand));
    itertools::assert_equal(map.iter().map(|(k, v)| (*k, *v)), VEC.map(expand));
    assert_eq!(map.query_one(KEY), Some(&expand(KEY).1));
    itertools::assert_equal(
        map.as_vals().iter().copied(),
        VEC.map(expand).map(|(_, v)| v),
    );
    assert_eq!(map.len() as usize, VEC.len());
}

#[test]
fn delete_all() {
    #[cfg(feature = "volatile")]
    let mut map = SparMap::<_, _>::new(Key::MAX as usize);

    #[cfg(feature = "memmap2")]
    let mut map = {
        const PATH: &str = "map_uge.delete_all.bin";

        let file = std::fs::File::create_new(PATH)
            .and_then(|f| {
                f.set_len(SparMap::<Key, u64>::file_size(Key::MAX as usize))?;
                Ok(f)
            })
            .or(std::fs::File::options().read(true).write(true).open(PATH))
            .unwrap();
        SparMap::<_, _>::from_buf(Key::MAX as usize, file)
    };

    map.insert_all(VEC.map(expand));
    map.delete_all(VEC.take(VEC.len() - 1));
    assert_eq!(map.as_vals(), &[KEY as u64]);
    map.delete_all(VEC.take(VEC.len() - 1));
    assert_eq!(map.as_vals(), &[KEY as u64]);
    assert_eq!(map.query_one(KEY), Some(&expand(KEY).1));
    #[cfg(not(feature = "rayon"))]
    itertools::assert_equal(map.query_all(VEC), std::iter::once(&expand(KEY).1));
    #[cfg(feature = "rayon")]
    assert_eq!(&*map.query_all(VEC).collect::<Box<_>>(), [&expand(KEY).1]);
    assert_eq!(map.len(), 1);
}
