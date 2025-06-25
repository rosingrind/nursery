use itertools::Itertools;
use sprs::{
    KEY_MAX, Key,
    map::{MapMut, MapRef, SparMap},
};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

#[test]
fn insert_all() {
    const VEC: std::ops::Range<Key> = 0..Key::MAX;

    let mut map = SparMap::<_, _>::new(KEY_MAX);
    let tmp = VEC.map(|x| (x, x.to_string())).collect::<Vec<_>>();
    let all = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Vec<_>>();

    map.insert_all(all.clone());
    assert_eq!(map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(), all);
    map.insert_all(all.clone());
    assert_eq!(map.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>(), all);
    assert_eq!(map.query_one(VEC.end - 1), Some(&"65534"));
    assert_eq!(
        map.as_vals(),
        all.iter().map(|(_, v)| *v).collect::<Vec<_>>()
    );
    assert_eq!(map.len(), VEC.end);
}

#[test]
fn delete_all() {
    const VEC_A: std::ops::Range<Key> = 0..Key::MAX;
    const VEC_B: std::ops::Range<Key> = 0..Key::MAX - 1;
    const KEY_TMP: usize = VEC_A.end as usize - 1;

    let mut map = SparMap::<_, _>::new(KEY_MAX);
    let tmp = VEC_A.map(|x| (x, x.to_string())).collect::<Box<_>>();
    let all = tmp
        .iter()
        .map(|(k, v)| (*k, v.as_str()))
        .collect::<Box<_>>();

    map.insert_all(all.clone());
    map.delete_all(VEC_B);
    assert_eq!(map.as_vals(), [all[KEY_TMP].1]);
    map.delete_all(VEC_B);
    assert_eq!(map.as_vals(), [all[KEY_TMP].1]);
    assert_eq!(map.query_one(VEC_A.end - 1), Some(&"65534"));
    assert_eq!(map.query_all(VEC_A).collect::<Vec<_>>(), [&"65534"]);
    assert_eq!(map.len(), 1);
}
