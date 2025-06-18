use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const FULL_RANGE: std::ops::Range<Key> = 0..Key::MAX;

#[test]
fn insert_all() {
    let mut set = SparSet::<Key, KEY_MAX>::new();

    set.insert_all(FULL_RANGE.collect::<Vec<_>>());
    assert_eq!(set.as_slice(), &FULL_RANGE.collect::<Vec<_>>(),);
    set.insert_all(FULL_RANGE.collect::<Vec<_>>());
    assert_eq!(set.as_slice(), &FULL_RANGE.collect::<Vec<_>>(),);
    assert_eq!(set.as_index_one(Key::MAX - 1), Some(65534));
    assert_eq!(set.as_slice(), &FULL_RANGE.collect::<Vec<_>>());
    assert_eq!(set.len(), Key::MAX);
}

#[test]
fn delete_all() {
    let mut set = SparSet::<Key, KEY_MAX>::new();

    set.insert_all(FULL_RANGE.collect::<Vec<_>>());
    set.delete_all(FULL_RANGE.take(Key::MAX as usize - 1).collect::<Vec<_>>());
    assert_eq!(set.as_slice(), &[Key::MAX - 1]);
    set.delete_all(FULL_RANGE.take(Key::MAX as usize - 1).collect::<Vec<_>>());
    assert_eq!(set.as_slice(), &[Key::MAX - 1]);
    assert_eq!(set.as_index_one(Key::MAX - 1), Some(0));
    assert_eq!(set.as_index_all(set.as_slice()).collect::<Vec<_>>(), [0]);
    assert_eq!(set.len(), 1);
}
