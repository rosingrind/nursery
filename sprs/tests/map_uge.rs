use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, MapRef, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

const FULL_RANGE: std::ops::Range<Key> = 0..Key::MAX;

#[test]
fn insert_all() {
    let mut map = SparMap::<_, _>::new(KEY_MAX);

    let tmp = FULL_RANGE.map(|x| (x, x.to_string())).collect::<Vec<_>>();
    let all = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();
    map.insert_all(all.clone());
    assert_eq!(map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(), all);
    map.insert_all(all.clone());
    assert_eq!(map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(), all);
    assert_eq!(map.query_one(Key::MAX - 1), Some(&"65534"));
    assert_eq!(
        map.as_vals(),
        all.iter().map(|(_, v)| *v).collect::<Vec<_>>()
    );
    assert_eq!(map.len(), Key::MAX);
}

#[test]
fn delete_all() {
    const KEY_TMP: usize = KEY_MAX - 1;

    let mut map = SparMap::<_, _>::new(KEY_MAX);
    let vec_a = FULL_RANGE.collect_array::<KEY_MAX>().unwrap();
    let vec_b = FULL_RANGE
        .take(Key::MAX as usize - 1)
        .collect_array::<KEY_TMP>()
        .unwrap();

    let tmp = FULL_RANGE.map(|x| (x, x.to_string())).collect::<Vec<_>>();
    let all = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();
    map.insert_all(all.clone());
    map.delete_all(vec_b);
    assert_eq!(map.as_vals(), [all[Key::MAX as usize - 1].1]);
    map.delete_all(vec_b);
    assert_eq!(map.as_vals(), [all[Key::MAX as usize - 1].1]);
    assert_eq!(map.query_one(Key::MAX - 1), Some(&"65534"));
    assert_eq!(map.query_all(&vec_a).collect::<Vec<_>>(), [&"65534"]);
    assert_eq!(map.len(), 1);
}
