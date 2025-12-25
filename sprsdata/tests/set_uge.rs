use sprsdata::*;

#[cfg(feature = "rayon")]
#[allow(unused_imports)]
use rayon::prelude::*;

const VEC: std::ops::Range<Key> = 0..Key::MAX;
const KEY: Key = VEC.end - 1;

#[test]
fn insert_all() {
    #[cfg(feature = "volatile")]
    let mut set = SparSet::<Key>::new(Key::MAX as usize);

    #[cfg(feature = "memmap2")]
    let mut set = {
        const PATH: &str = "set_uge.insert_all.bin";

        let file = std::fs::File::create_new(PATH)
            .and_then(|f| {
                f.set_len(SparSet::<Key>::file_size(Key::MAX as usize))?;
                Ok(f)
            })
            .or(std::fs::File::options().read(true).write(true).open(PATH))
            .unwrap();
        SparSet::<Key>::from_buf(Key::MAX as usize, file)
    };

    set.insert_all(VEC);
    itertools::assert_equal(set.iter().copied(), VEC);
    set.insert_all(VEC);
    itertools::assert_equal(set.iter().copied(), VEC);
    assert_eq!(set.as_index_one(KEY), Some(KEY));
    itertools::assert_equal(set.as_slice().iter().copied(), VEC);
    assert_eq!(set.len() as usize, VEC.len());
}

#[test]
fn delete_all() {
    #[cfg(feature = "volatile")]
    let mut set = SparSet::<Key>::new(Key::MAX as usize);

    #[cfg(feature = "memmap2")]
    let mut set = {
        const PATH: &str = "set_uge.delete_all.bin";

        let file = std::fs::File::create_new(PATH)
            .and_then(|f| {
                f.set_len(SparSet::<Key>::file_size(Key::MAX as usize))?;
                Ok(f)
            })
            .or(std::fs::File::options().read(true).write(true).open(PATH))
            .unwrap();
        SparSet::<Key>::from_buf(Key::MAX as usize, file)
    };

    set.insert_all(VEC);
    set.delete_all(VEC.take(VEC.len() - 1));
    assert_eq!(set.as_slice(), &[KEY]);
    set.delete_all(VEC.take(VEC.len() - 1));
    assert_eq!(set.as_slice(), &[KEY]);
    assert_eq!(set.as_index_one(KEY), Some(0));
    #[cfg(not(feature = "rayon"))]
    itertools::assert_equal(set.as_index_all(VEC), std::iter::once(0));
    #[cfg(feature = "rayon")]
    assert_eq!(&*set.as_index_all(VEC).collect::<Box<_>>(), [0]);
    assert_eq!(set.len(), 1);
}
