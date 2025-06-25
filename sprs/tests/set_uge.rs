use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    set::{SetMut, SetRef, SparSet},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[test]
fn insert_all() {
    const VEC: std::ops::Range<Key> = 0..Key::MAX;

    let mut set = SparSet::<Key>::new(KEY_MAX);
    let vec = VEC.collect_array::<KEY_MAX>().unwrap();

    set.insert_all(VEC);
    assert_eq!(set.as_slice(), vec);
    set.insert_all(VEC);
    assert_eq!(set.as_slice(), vec);
    assert_eq!(set.as_index_one(VEC.end - 1), Some(65534));
    assert_eq!(set.as_slice(), vec);
    assert_eq!(set.len(), Key::MAX);
}

#[test]
fn delete_all() {
    const VEC_A: std::ops::Range<Key> = 0..Key::MAX;
    const VEC_B: std::ops::Range<Key> = 0..Key::MAX - 1;

    let mut set = SparSet::<Key>::new(KEY_MAX);

    set.insert_all(VEC_A);
    set.delete_all(VEC_B);
    assert_eq!(set.as_slice(), &[VEC_A.end - 1]);
    set.delete_all(VEC_B);
    assert_eq!(set.as_slice(), &[VEC_A.end - 1]);
    assert_eq!(set.as_index_one(VEC_A.end - 1), Some(0));
    assert_eq!(
        set.as_index_all(set.as_slice().to_vec())
            .collect::<Vec<_>>(),
        [0]
    );
    assert_eq!(set.len(), 1);
}
