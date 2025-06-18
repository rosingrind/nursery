mod impl_mut;
mod impl_ops;
mod impl_ref;

use std::alloc::Layout;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub use impl_mut::MapMut;
pub use impl_ref::MapRef;

use crate::{
    Key,
    set::{SetRef, SparSet, alloc_slice},
};

pub type MapSlice<T> = [T; Key::MAX as usize];
type MapSliceMask = bitvec::BitArr!(for Key::MAX as usize, in Key);

// TOOD: sorted criteria; for example you can store `Pos { x, y }`
// as `x` and `y`, and sort each other - this allows to query
// by position (find all by `x` in range, by `y` in range, intersect results)
#[cfg_attr(feature = "bitcode", derive(Decode, Encode))]
#[derive(Clone)]
pub struct SparMap<T> {
    keys: SparSet,
    // TOOD: a generic storage (availability to store GPU buffer slice instead of this)
    vals: Box<[T]>,
}

impl<T: Send + Sync + Copy> Default for SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Send + Sync + Copy> SparMap<T> {
    #[cfg_attr(feature = "inline-more", inline)]
    pub fn new() -> Self {
        Self {
            keys: SparSet::new(),
            vals: alloc_slice(Layout::new::<MapSlice<T>>()),
        }
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn len(&self) -> Key {
        self.keys.len()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys(&self) -> &[Key] {
        self.keys.as_slice()
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_keys_set(&self) -> &impl SetRef {
        &self.keys
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals(&self) -> &[T] {
        let len = self.len() as usize;
        &self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn as_vals_mut(&mut self) -> &mut [T] {
        let len = self.len() as usize;
        &mut self.vals[..len]
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one(&self, k: Key) -> Option<&T> {
        self.keys.as_index_one(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn query_one_mut(&mut self, k: Key) -> Option<&mut T> {
        self.keys
            .as_index_one(k)
            .map(|k| &mut self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all(&self, k: &[Key]) -> impl Iterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all(&self, k: &[Key]) -> impl ParallelIterator<Item = &T> {
        self.keys.as_index_all(k).map(|k| &self.vals[k as usize])
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(not(feature = "rayon"))]
    pub fn query_all_mut(&mut self, k: &[Key]) -> impl Iterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k as usize) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    #[cfg(feature = "rayon")]
    pub fn query_all_mut(&mut self, k: &[Key]) -> impl ParallelIterator<Item = &mut T> {
        self.keys.as_index_all(k).map(|k| {
            let ptr = self.vals.as_ptr();
            let raw = ptr as *mut T;
            unsafe { &mut *raw.add(k as usize) }
        })
    }

    #[cfg_attr(feature = "inline-more", inline)]
    pub fn contains(&self, i: Key) -> bool {
        self.keys.contains(i)
    }

    pub(crate) fn filter_all_excl(&self, kv: &[(Key, T)]) -> (Vec<Key>, Vec<T>) {
        let mut bit = MapSliceMask::ZERO;
        let mut k = Vec::with_capacity(kv.len());
        let mut v = Vec::with_capacity(kv.len());

        for (i, x) in kv.iter() {
            if !bit[*i as usize] && !self.keys.contains(*i) {
                bit.set(*i as usize, true);
                k.push(*i);
                v.push(*x);
            }
        }

        (k, v)
    }
}

impl<'a, T> IntoIterator for &'a SparMap<T> {
    type Item = (&'a Key, &'a T);
    type IntoIter = impl_ref::MapIter<'a, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut SparMap<T> {
    type Item = (&'a Key, &'a T);
    type IntoIter = impl_ref::MapIter<'a, T>;

    #[cfg_attr(feature = "inline-more", inline)]
    fn into_iter(self) -> impl_ref::MapIter<'a, T> {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_ops() {
        let mut map = SparMap::new();

        assert!(map.vals[..map.len() as usize].is_empty());
        assert!(map.as_vals().is_empty());
        assert_eq!(map.len(), 0);

        assert_eq!(map.insert_one(5, "0"), None);
        assert_eq!(map.insert_one(5, "0"), Some("0"));
        assert!(map.contains(5));
        assert_eq!(map.vals[..map.len() as usize], ["0"]);
        assert_eq!(map.as_vals(), ["0"]);
        assert_eq!(map.query_one(5), Some(&"0"));
        assert_eq!(map.query_all(&[5]).collect::<Vec<_>>(), [&"0"]);
        assert_eq!(map.len(), 1);

        assert!(map.delete_one(5).is_some());
        assert!(map.delete_one(5).is_none());
        assert!(!map.contains(5));
        assert!(map.vals[..map.len() as usize].is_empty());
        assert!(map.as_vals().is_empty());
        assert_eq!(map.query_one(5), None);
        assert_eq!(map.query_all(&[5]).count(), 0);
        assert_eq!(map.len(), 0);

        let range = (4..8).map(|x| (x, x.to_string())).collect::<Vec<_>>();
        for (i, (k, v)) in range.iter().enumerate() {
            assert_eq!(map.insert_one(*k, v.as_str()), None);

            assert!(map.contains(*k));
            assert_eq!(map.vals[i], v.as_str());
            assert_eq!(map.as_vals()[i], v.as_str());
            assert_eq!(map.query_one(*k), Some(&v.as_str()));
            assert_eq!(map.query_all(&[*k]).collect::<Vec<_>>(), [&v.as_str()]);
            assert_eq!(map.len(), i as Key + 1);
        }
        assert!(map.query_one(3).is_none());
        assert!(map.query_one(8).is_none());
        assert_eq!(map.delete_one(5), Some("5"));
        assert_eq!(map.as_vals(), &["4", "7", "6"]);
        assert_eq!(map.delete_one(6), Some("6"));
        assert_eq!(map.as_vals(), &["4", "7"]);
        assert_eq!(map.delete_one(4), Some("4"));
        assert_eq!(map.as_vals(), &["7"]);
        assert_eq!(map.delete_one(7), Some("7"));
        assert!(map.as_vals().is_empty());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn batched_ops() {
        let mut map = SparMap::new();

        map.insert_all(vec![(4, "0"), (5, "1"), (6, "2"), (7, "3")]);
        assert_eq!(
            map.iter().collect::<Vec<_>>(),
            vec![(&4, &"0"), (&5, &"1"), (&6, &"2"), (&7, &"3")]
        );
        map.insert_all(vec![(4, "0"), (5, "1"), (6, "2"), (7, "3")]);
        assert_eq!(
            map.iter().collect::<Vec<_>>(),
            vec![(&4, &"0"), (&5, &"1"), (&6, &"2"), (&7, &"3")]
        );
        assert_eq!(map.as_vals(), &["0", "1", "2", "3"]);
        assert_eq!(
            map.query_all(&[5, 4, 7, 6]).collect::<Vec<_>>(),
            [&"1", &"0", &"3", &"2"]
        );
        assert_eq!(map.len(), 4);

        map.delete_all(&[5, 4, 7, 2]);
        assert_eq!(map.as_vals(), &["2"]);
        map.delete_all(&[5, 4, 7, 2]);
        assert_eq!(map.as_vals(), &["2"]);
        assert_eq!(map.query_one(6), Some(&"2"));
        assert_eq!(map.query_all(&[6]).collect::<Vec<_>>(), [&"2"]);
        assert_eq!(map.len(), 1);
    }
}

#[cfg(test)]
mod test_map {
    use super::*;

    use rand::{Rng, SeedableRng, rngs::SmallRng};
    use std::borrow::ToOwned;
    use std::cell::RefCell;
    use std::vec::Vec;

    #[test]
    fn test_zero_capacities() {
        type HM = SparMap<i32>;

        let m = HM::new();
        assert_eq!(m.len(), 0);

        let m = HM::default();
        assert_eq!(m.len(), 0);

        let mut m = HM::new();
        m.insert_one(1, 1);
        m.insert_one(2, 2);
        m.delete_one(1);
        m.delete_one(2);
        assert_eq!(m.len(), 0);

        let m = HM::new();
        assert_eq!(m.len(), 0);
    }

    #[test]
    fn test_create_capacity_zero() {
        let mut m = SparMap::default();

        assert!(m.insert_one(1, 1).is_none());

        assert!(m.contains(1));
        assert!(!m.contains(0));
    }

    #[test]
    fn test_insert() {
        let mut m = SparMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert_one(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert_one(2, 4).is_none());
        assert_eq!(m.len(), 2);
        assert_eq!(*m.query_one(1).unwrap(), 2);
        assert_eq!(*m.query_one(2).unwrap(), 4);
    }

    #[test]
    fn test_clone() {
        let mut m = SparMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert_one(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert_one(2, 4).is_none());
        assert_eq!(m.len(), 2);
        #[allow(clippy::redundant_clone)]
        let m2 = m.clone();
        assert_eq!(*m2.query_one(1).unwrap(), 2);
        assert_eq!(*m2.query_one(2).unwrap(), 4);
        assert_eq!(m2.len(), 2);
    }

    #[test]
    fn test_clone_from() {
        let mut m = SparMap::new();
        let mut m2 = SparMap::new();
        assert_eq!(m.len(), 0);
        assert!(m.insert_one(1, 2).is_none());
        assert_eq!(m.len(), 1);
        assert!(m.insert_one(2, 4).is_none());
        assert_eq!(m.len(), 2);
        m2.clone_from(&m);
        assert_eq!(*m2.query_one(1).unwrap(), 2);
        assert_eq!(*m2.query_one(2).unwrap(), 4);
        assert_eq!(m2.len(), 2);
    }

    thread_local! { static DROP_VECTOR: RefCell<Vec<i32>> = const { RefCell::new(Vec::new()) } }

    #[derive(Hash, PartialEq, Eq)]
    struct Droppable {
        k: usize,
    }

    impl Droppable {
        fn new(k: usize) -> Droppable {
            DROP_VECTOR.with(|slot| {
                slot.borrow_mut()[k] += 1;
            });

            Droppable { k }
        }
    }

    impl Drop for Droppable {
        fn drop(&mut self) {
            DROP_VECTOR.with(|slot| {
                slot.borrow_mut()[self.k] -= 1;
            });
        }
    }

    impl Clone for Droppable {
        fn clone(&self) -> Self {
            Droppable::new(self.k)
        }
    }

    #[test]
    fn test_empty_remove() {
        let mut m: SparMap<i32> = SparMap::new();
        assert_eq!(m.delete_one(0), None);
    }

    #[test]
    fn test_empty_iter() {
        let mut m: SparMap<i32> = SparMap::new();
        assert_eq!(m.iter().next(), None);
        assert_eq!(m.as_keys().iter().next(), None);
        assert_eq!(m.as_vals().iter().next(), None);
        assert_eq!(m.as_keys_set().iter().next(), None);
        assert_eq!(m.as_vals_mut().iter().next(), None);
        assert_eq!(m.len(), 0);
        assert!(m.is_empty());
    }

    #[test]
    #[cfg_attr(miri, ignore)] // FIXME: takes too long
    fn test_lots_of_insertions() {
        let mut m = SparMap::new();

        // Try this a few times to make sure we never screw up the hashmap's
        // internal state.
        for _ in 0..10 {
            assert!(m.is_empty());

            for i in 1..1001 {
                assert!(m.insert_one(i, i).is_none());

                for j in 1..=i {
                    let r = m.query_one(j);
                    assert_eq!(r, Some(&j));
                }

                for j in i + 1..1001 {
                    let r = m.query_one(j);
                    assert_eq!(r, None);
                }
            }

            for i in 1001..2001 {
                assert!(!m.contains(i));
            }

            // remove forwards
            for i in 1..1001 {
                assert!(m.delete_one(i).is_some());

                for j in 1..=i {
                    assert!(!m.contains(j));
                }

                for j in i + 1..1001 {
                    assert!(m.contains(j));
                }
            }

            for i in 1..1001 {
                assert!(!m.contains(i));
            }

            for i in 1..1001 {
                assert!(m.insert_one(i, i).is_none());
            }

            // remove backwards
            for i in (1..1001).rev() {
                assert!(m.delete_one(i).is_some());

                for j in i..1001 {
                    assert!(!m.contains(j));
                }

                for j in 1..i {
                    assert!(m.contains(j));
                }
            }
        }
    }

    #[test]
    fn test_find_mut() {
        let mut m = SparMap::new();
        assert!(m.insert_one(1, 12).is_none());
        assert!(m.insert_one(2, 8).is_none());
        assert!(m.insert_one(5, 14).is_none());
        let new = 100;
        match m.query_one_mut(5) {
            None => panic!(),
            Some(x) => *x = new,
        }
        assert_eq!(m.query_one(5), Some(&new));
    }

    #[test]
    fn test_insert_overwrite() {
        let mut m = SparMap::new();
        assert!(m.insert_one(1, 2).is_none());
        assert_eq!(*m.query_one(1).unwrap(), 2);
        assert!(m.insert_one(1, 3).is_some());
        assert_eq!(*m.query_one(1).unwrap(), 3);
    }

    #[test]
    fn test_insert_conflicts() {
        let mut m = SparMap::new();
        assert!(m.insert_one(1, 2).is_none());
        assert!(m.insert_one(5, 3).is_none());
        assert!(m.insert_one(9, 4).is_none());
        assert_eq!(*m.query_one(9).unwrap(), 4);
        assert_eq!(*m.query_one(5).unwrap(), 3);
        assert_eq!(*m.query_one(1).unwrap(), 2);
    }

    #[test]
    fn test_conflict_remove() {
        let mut m = SparMap::new();
        assert!(m.insert_one(1, 2).is_none());
        assert_eq!(*m.query_one(1).unwrap(), 2);
        assert!(m.insert_one(5, 3).is_none());
        assert_eq!(*m.query_one(1).unwrap(), 2);
        assert_eq!(*m.query_one(5).unwrap(), 3);
        assert!(m.insert_one(9, 4).is_none());
        assert_eq!(*m.query_one(1).unwrap(), 2);
        assert_eq!(*m.query_one(5).unwrap(), 3);
        assert_eq!(*m.query_one(9).unwrap(), 4);
        assert!(m.delete_one(1).is_some());
        assert_eq!(*m.query_one(9).unwrap(), 4);
        assert_eq!(*m.query_one(5).unwrap(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut m = SparMap::new();
        assert!(m.insert_one(1, 2).is_none());
        assert!(!m.is_empty());
        assert!(m.delete_one(1).is_some());
        assert!(m.is_empty());
    }

    #[test]
    fn test_remove() {
        let mut m = SparMap::new();
        m.insert_one(1, 2);
        assert_eq!(m.delete_one(1), Some(2));
        assert_eq!(m.delete_one(1), None);
    }

    #[test]
    fn test_remove_entry() {
        let mut m = SparMap::new();
        m.insert_one(1, 2);
        assert_eq!(m.delete_one(1), Some(2));
        assert_eq!(m.delete_one(1), None);
    }

    #[test]
    fn test_iterate() {
        let mut m = SparMap::new();
        for i in 0..32 {
            assert!(m.insert_one(i, i * 2).is_none());
        }
        assert_eq!(m.len(), 32);

        let mut observed: u32 = 0;

        for (k, v) in m.iter() {
            assert_eq!(*v, *k * 2);
            observed |= 1 << *k;
        }
        assert_eq!(observed, 0xFFFF_FFFF);
    }

    #[test]
    fn test_keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: SparMap<_> = vec.into_iter().collect();
        let keys: Vec<_> = map.as_keys().to_vec();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: SparMap<_> = vec.into_iter().collect();
        let values: Vec<_> = map.as_vals().to_vec();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_values_mut() {
        let vec = vec![(1, 1), (2, 2), (3, 3)];
        let mut map: SparMap<_> = vec.into_iter().collect();
        for value in map.as_vals_mut() {
            *value *= 2;
        }
        let values: Vec<_> = map.as_vals().to_vec();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&2));
        assert!(values.contains(&4));
        assert!(values.contains(&6));
    }

    #[test]
    fn test_into_keys() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: SparMap<_> = vec.into_iter().collect();
        let keys: Vec<_> = map.as_keys().to_vec();

        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_into_values() {
        let vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        let map: SparMap<_> = vec.into_iter().collect();
        let values: Vec<_> = map.as_vals().to_vec();

        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_find() {
        let mut m = SparMap::new();
        assert!(m.query_one(1).is_none());
        m.insert_one(1, 2);
        match m.query_one(1) {
            None => panic!(),
            Some(v) => assert_eq!(*v, 2),
        }
    }

    #[test]
    fn test_eq() {
        let mut m1 = SparMap::new();
        m1.insert_one(1, 2);
        m1.insert_one(2, 3);
        m1.insert_one(3, 4);

        let mut m2 = SparMap::new();
        m2.insert_one(1, 2);
        m2.insert_one(2, 3);

        assert!(m1 != m2);

        m2.insert_one(3, 4);

        assert_eq!(m1, m2);
    }

    #[test]
    fn test_show() {
        let mut map = SparMap::new();
        let empty: SparMap<i32> = SparMap::new();

        map.insert_one(1, 2);
        map.insert_one(3, 4);

        let map_str = format!("{map:?}");

        assert!(map_str == "{1: 2, 3: 4}" || map_str == "{3: 4, 1: 2}");
        assert_eq!(format!("{empty:?}"), "{}");
    }

    #[test]
    fn test_from_iter() {
        let xs = [(1, 1), (2, 2), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: SparMap<_> = xs.iter().copied().collect();

        for &(k, v) in &xs {
            assert_eq!(map.query_one(k), Some(&v));
        }

        assert_eq!(map.iter().len(), xs.len() - 1);
    }

    #[test]
    fn test_size_hint() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: SparMap<_> = xs.iter().copied().collect();

        let mut iter = map.iter();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.size_hint(), (3, Some(3)));
    }

    #[test]
    fn test_iter_len() {
        let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

        let map: SparMap<_> = xs.iter().copied().collect();

        let mut iter = map.iter();

        for _ in iter.by_ref().take(3) {}

        assert_eq!(iter.len(), 3);
    }

    // #[test]
    // fn test_mut_size_hint() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    //     let mut map: SparMap<_> = xs.iter().copied().collect();

    //     let mut iter = map.iter_mut();

    //     for _ in iter.by_ref().take(3) {}

    //     assert_eq!(iter.size_hint(), (3, Some(3)));
    // }

    // #[test]
    // fn test_iter_mut_len() {
    //     let xs = [(1, 1), (2, 2), (3, 3), (4, 4), (5, 5), (6, 6)];

    //     let mut map: SparMap<_> = xs.iter().copied().collect();

    //     let mut iter = map.iter_mut();

    //     for _ in iter.by_ref().take(3) {}

    //     assert_eq!(iter.len(), 3);
    // }

    #[test]
    fn test_index() {
        let mut map = SparMap::new();

        map.insert_one(1, 2);
        map.insert_one(2, 1);
        map.insert_one(3, 4);

        assert_eq!(map[2], 1);
    }

    #[test]
    #[should_panic]
    fn test_index_nonexistent() {
        let mut map = SparMap::new();

        map.insert_one(1, 2);
        map.insert_one(2, 1);
        map.insert_one(3, 4);

        #[allow(clippy::no_effect)] // false positive lint
        map[4];
    }

    #[test]
    fn test_extend_ref_k_ref_v() {
        let mut a = SparMap::new();
        a.insert_one(1, "one");
        let mut b = SparMap::new();
        b.insert_one(2, "two");
        b.insert_one(3, "three");

        a.extend(b.iter());

        assert_eq!(a.len(), 3);
        assert_eq!(a[1], "one");
        assert_eq!(a[2], "two");
        assert_eq!(a[3], "three");
    }

    #[test]
    #[allow(clippy::needless_borrow)]
    fn test_extend_ref_kv_tuple() {
        use std::ops::AddAssign;
        let mut a = SparMap::new();
        a.insert_one(0, 0);

        fn create_arr<T: AddAssign<T> + Copy, const N: usize>(start: T, step: T) -> [(T, T); N] {
            let mut outs: [(T, T); N] = [(start, start); N];
            let mut element = step;
            outs.iter_mut().skip(1).for_each(|(k, v)| {
                *k += element;
                *v += element;
                element += step;
            });
            outs
        }

        let for_iter: Vec<_> = (0..100).map(|i| (i, i)).collect();
        let iter = for_iter.iter();
        let vec: Vec<_> = (100..200).map(|i| (i, i)).collect();
        a.extend(iter);
        a.extend(&vec);
        a.extend(create_arr::<Key, 100>(200, 1));

        assert_eq!(a.len(), 300);

        for item in 0..300 {
            assert_eq!(a[item], item);
        }
    }

    #[test]
    fn test_replace_entry_with_doesnt_corrupt() {
        let mut m = SparMap::new();

        let mut rng = {
            let seed = u64::from_le_bytes(*b"testseed");
            SmallRng::seed_from_u64(seed)
        };

        // Populate the map with some items.
        for _ in 0..50 {
            let x = rng.gen_range(0..20);
            m.insert_one(x, ());
        }
    }

    #[test]
    fn test_retain() {
        let mut map: SparMap<i32> = (0..100).map(|x| (x as Key, x * 10)).collect();

        map.retain(|&k, _| k % 2 == 0);
        assert_eq!(map.len(), 50);
        assert_eq!(map[2], 20);
        assert_eq!(map[4], 40);
        assert_eq!(map[6], 60);
    }

    #[test]
    fn test_recall() {
        {
            let mut map: SparMap<i32> = (0..8).map(|x| (x as Key, x * 10)).collect();
            let drained = map.recall(|&k, _| k % 2 == 0);
            let mut out = drained.collect::<Vec<_>>();
            out.sort_unstable();
            assert_eq!(vec![0, 20, 40, 60], out);
            assert_eq!(map.len(), 4);
        }
        {
            let mut map: SparMap<i32> = (0..8).map(|x| (x as Key, x * 10)).collect();
            map.recall(|&k, _| k % 2 == 0).for_each(drop);
            assert_eq!(map.len(), 4);
        }
    }

    #[test]
    fn test_get_many_mut() {
        let mut map = SparMap::new();
        map.insert_one(0, "foo");
        map.insert_one(10, "bar");
        map.insert_one(20, "baz");
        map.insert_one(30, "qux");

        let xs = map.query_all_mut(&[0, 30]);
        assert_eq!(xs.collect::<Vec<_>>(), vec![&mut "foo", &mut "qux"]);

        let xs = map.query_all_mut(&[0, 5]);
        assert_eq!(xs.collect::<Vec<_>>(), vec![&mut "foo"]);
    }
}
