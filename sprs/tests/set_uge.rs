use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const FULL_RANGE: std::ops::Range<Key> = 0..Key::MAX;

#[test]
fn insert_all() {
    let mut set = SparSet::<Key>::new(KEY_MAX);
    let vec = FULL_RANGE.collect_array::<KEY_MAX>().unwrap();

    set.insert_all(&vec);
    assert_eq!(set.as_slice(), &vec);
    set.insert_all(&vec);
    assert_eq!(set.as_slice(), &vec);
    assert_eq!(set.as_index_one(Key::MAX - 1), Some(65534));
    assert_eq!(set.as_slice(), &vec);
    assert_eq!(set.len(), Key::MAX);
}

#[test]
fn delete_all() {
    const KEY_TMP: usize = KEY_MAX - 1;

    let mut set = SparSet::<Key>::new(KEY_MAX);
    let vec_a = FULL_RANGE.collect_array::<KEY_MAX>().unwrap();
    let vec_b = FULL_RANGE
        .take(Key::MAX as usize - 1)
        .collect_array::<KEY_TMP>()
        .unwrap();

    set.insert_all(&vec_a);
    set.delete_all(&vec_b);
    assert_eq!(set.as_slice(), &[Key::MAX - 1]);
    set.delete_all(&vec_b);
    assert_eq!(set.as_slice(), &[Key::MAX - 1]);
    assert_eq!(set.as_index_one(Key::MAX - 1), Some(0));
    assert_eq!(set.as_index_all(set.as_slice()).collect::<Vec<_>>(), [0]);
    assert_eq!(set.len(), 1);
}
