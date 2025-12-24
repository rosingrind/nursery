use crate::Key;

use super::*;

type MockSet = SparSet<Key>;

#[test]
fn regular_ops() {
    let mut set = MockSet::new(Key::MAX as usize);

    assert_eq!(set.d()[..*set.l() as usize], []);
    assert_eq!(set.as_slice(), &[]);
    assert_eq!(set.as_index_all([]).count(), 0);
    assert_eq!(set.as_index_all([1, 2, 3]).count(), 0);
    assert_eq!(*set.l(), 0);
    assert_eq!(set.len(), 0);

    assert!(set.insert_one(5));
    assert!(!set.insert_one(5));
    assert!(set.contains(5));
    assert_eq!(set.d()[..*set.l() as usize], [5]);
    assert_eq!(set.s()[5], 0);
    assert_eq!(set.as_slice(), &[5]);
    assert_eq!(set.as_index_one(5), Some(0));
    assert_eq!(set.as_index_all([5]).collect::<Vec<_>>(), [0]);
    assert_eq!(*set.l(), 1);
    assert_eq!(set.len(), *set.l());

    assert!(set.delete_one(5));
    assert!(!set.delete_one(5));
    assert!(!set.contains(5));
    assert_eq!(set.d()[..*set.l() as usize], []);
    assert_eq!(set.as_slice(), &[]);
    assert_eq!(set.as_index_one(5), None);
    assert_eq!(set.as_index_all([5]).count(), 0);
    assert_eq!(*set.l(), 0);
    assert_eq!(set.len(), *set.l());

    for (i, k) in (4..8).enumerate() {
        assert!(set.insert_one(k));
        assert_eq!(set.d()[i], k);
        assert_eq!(set.s()[k as usize], i as Key);
        assert_eq!(set.as_slice()[i], k);
        assert_eq!(set.as_index_one(k), Some(i as Key));
        assert_eq!(set.as_index_all([k]).collect::<Vec<_>>(), [i as Key]);
        assert_eq!(*set.l(), i as Key + 1);
        assert_eq!(set.len(), *set.l());
    }
    assert!(set.as_index_one(3).is_none());
    assert!(set.as_index_one(8).is_none());
    assert!(set.delete_one(5));
    assert_eq!(set.as_slice(), &[4, 7, 6]);
    assert!(set.delete_one(6));
    assert_eq!(set.as_slice(), &[4, 7]);
    assert!(set.delete_one(4));
    assert_eq!(set.as_slice(), &[7]);
    assert!(set.delete_one(7));
    assert_eq!(set.as_slice(), &[]);
    assert_eq!(*set.l(), 0);
    assert_eq!(set.len(), *set.l());
}

#[test]
fn compare_ops() {
    let mut a = MockSet::new(Key::MAX as usize);
    let mut b = MockSet::new(Key::MAX as usize);

    a.insert_all([5, 2]);
    b.insert_all([5, 3]);

    assert_eq!(vec![&5], {
        let mut x = a.intersection(&b).collect::<Vec<_>>();
        // let mut y = &a & &b;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });
    // assert!(!(&a & &b).contains(&2));
    // assert!(!(&a & &b).contains(&3));

    assert_eq!(vec![&2, &3, &5], {
        let mut x = a.union(&b).collect::<Vec<_>>();
        // let mut y = &a | &b;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });

    assert_eq!(vec![&2], {
        let mut x = a.difference(&b).collect::<Vec<_>>();
        // let mut y = &a - &b;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });
    assert_eq!(vec![&3], {
        let mut x = b.difference(&a).collect::<Vec<_>>();
        // let mut y = &b - &a;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });

    assert_eq!(vec![&2, &3], {
        let mut x = a.symmetric_difference(&b).collect::<Vec<_>>();
        // let mut y = &a - &b;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });
    assert_eq!(vec![&2, &3], {
        let mut x = b.symmetric_difference(&a).collect::<Vec<_>>();
        // let mut y = &b - &a;
        x.sort_unstable();
        // y.sort_unstable();
        // assert_eq!(x, y);
        x
    });
}

#[test]
fn batched_ops() {
    let mut set = MockSet::new(Key::MAX as usize);

    set.insert_all([4, 5, 6, 7]);
    assert_eq!(set.as_slice(), [4, 5, 6, 7]);
    set.insert_all([4, 5, 6, 7]);
    assert_eq!(set.as_slice(), [4, 5, 6, 7]);
    assert_eq!(set.as_index_one(6), Some(2));
    assert_eq!(
        set.as_index_all([5, 4, 7, 6]).collect::<Vec<_>>(),
        [1, 0, 3, 2]
    );
    assert_eq!(*set.l(), 4);
    assert_eq!(set.len(), *set.l());

    set.delete_all([5, 5, 5, 4, 4, 4, 7, 2, 2, 2, 5, 5, 5]);
    assert_eq!(set.as_slice(), [6]);
    set.delete_all([5, 5, 5, 4, 4, 4, 7, 2, 2, 2, 5, 5, 5]);
    assert_eq!(set.as_slice(), [6]);
    assert_eq!(set.as_index_one(6), Some(0));
    assert_eq!(
        set.as_index_all(set.as_slice().to_vec())
            .collect::<Vec<_>>(),
        [0]
    );
    assert_eq!(*set.l(), 1);
    assert_eq!(set.len(), *set.l());
}

#[test]
fn test_zero_capacities() {
    type HS = MockSet;

    let s = HS::new(Key::MAX as usize);
    assert_eq!(s.len(), 0);

    let s = HS::default();
    assert_eq!(s.len(), 0);

    let mut s = HS::new(Key::MAX as usize);
    s.insert_one(1);
    s.insert_one(2);
    s.delete_one(1);
    s.delete_one(2);
    assert_eq!(s.len(), 0);
}

#[test]
fn test_disjoint() {
    let mut xs = MockSet::new(Key::MAX as usize);
    let mut ys = MockSet::new(Key::MAX as usize);
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(xs.insert_one(5));
    assert!(ys.insert_one(11));
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(xs.insert_one(7));
    assert!(xs.insert_one(19));
    assert!(xs.insert_one(4));
    assert!(ys.insert_one(2));
    assert!(xs.is_disjoint(&ys));
    assert!(ys.is_disjoint(&xs));
    assert!(ys.insert_one(7));
    assert!(!xs.is_disjoint(&ys));
    assert!(!ys.is_disjoint(&xs));
}

#[test]
fn test_subset_and_superset() {
    let mut a = MockSet::new(Key::MAX as usize);
    assert!(a.insert_one(0));
    assert!(a.insert_one(5));
    assert!(a.insert_one(11));
    assert!(a.insert_one(7));

    let mut b = MockSet::new(Key::MAX as usize);
    assert!(b.insert_one(0));
    assert!(b.insert_one(7));
    assert!(b.insert_one(19));
    assert!(b.insert_one(250));
    assert!(b.insert_one(11));
    assert!(b.insert_one(200));

    assert!(!a.is_subset(&b));
    assert!(!a.is_superset(&b));
    assert!(!b.is_subset(&a));
    assert!(!b.is_superset(&a));

    assert!(b.insert_one(5));

    assert!(a.is_subset(&b));
    assert!(!a.is_superset(&b));
    assert!(!b.is_subset(&a));
    assert!(b.is_superset(&a));
}

#[test]
fn test_iterate() {
    let mut a = MockSet::new(Key::MAX as usize);
    for i in 0..32 {
        assert!(a.insert_one(i));
    }
    let mut observed: u32 = 0;
    for k in a.iter() {
        observed |= 1 << *k;
    }
    assert_eq!(observed, 0xFFFF_FFFF);
}

#[test]
fn test_intersection() {
    let mut a = MockSet::new(Key::MAX as usize);
    let mut b = MockSet::new(Key::MAX as usize);

    assert!(a.insert_one(11));
    assert!(a.insert_one(1));
    assert!(a.insert_one(3));
    assert!(a.insert_one(77));
    assert!(a.insert_one(103));
    assert!(a.insert_one(5));

    assert!(b.insert_one(2));
    assert!(b.insert_one(11));
    assert!(b.insert_one(77));
    assert!(b.insert_one(5));
    assert!(b.insert_one(3));

    let expected = [3, 5, 11, 77];
    let i = a
        .intersection(&b)
        .inspect(|x| assert!(expected.contains(x)))
        .count();
    assert_eq!(i, expected.len());
}

#[test]
fn test_difference() {
    let mut a = MockSet::new(Key::MAX as usize);
    let mut b = MockSet::new(Key::MAX as usize);

    assert!(a.insert_one(1));
    assert!(a.insert_one(3));
    assert!(a.insert_one(5));
    assert!(a.insert_one(9));
    assert!(a.insert_one(11));

    assert!(b.insert_one(3));
    assert!(b.insert_one(9));

    let expected = [1, 5, 11];
    let i = a
        .difference(&b)
        .inspect(|x| assert!(expected.contains(x)))
        .count();
    assert_eq!(i, expected.len());
}

#[test]
fn test_symmetric_difference() {
    let mut a = MockSet::new(Key::MAX as usize);
    let mut b = MockSet::new(Key::MAX as usize);

    assert!(a.insert_one(1));
    assert!(a.insert_one(3));
    assert!(a.insert_one(5));
    assert!(a.insert_one(9));
    assert!(a.insert_one(11));

    assert!(b.insert_one(3));
    assert!(b.insert_one(9));
    assert!(b.insert_one(14));
    assert!(b.insert_one(22));

    let expected = [1, 5, 11, 14, 22];
    let i = a
        .symmetric_difference(&b)
        .inspect(|x| assert!(expected.contains(x)))
        .count();
    assert_eq!(i, expected.len());
}

#[test]
fn test_union() {
    let mut a = MockSet::new(Key::MAX as usize);
    let mut b = MockSet::new(Key::MAX as usize);

    assert!(a.insert_one(1));
    assert!(a.insert_one(3));
    assert!(a.insert_one(5));
    assert!(a.insert_one(9));
    assert!(a.insert_one(11));
    assert!(a.insert_one(16));
    assert!(a.insert_one(19));
    assert!(a.insert_one(24));

    assert!(b.insert_one(1));
    assert!(b.insert_one(5));
    assert!(b.insert_one(9));
    assert!(b.insert_one(13));
    assert!(b.insert_one(19));

    let expected = [1, 3, 5, 9, 11, 13, 16, 19, 24];
    let i = a
        .union(&b)
        .inspect(|x| assert!(expected.contains(x)))
        .count();
    assert_eq!(i, expected.len());
}

#[test]
fn test_from_iter() {
    let xs = [1, 2, 2, 3, 4, 5, 6, 7, 8, 9];

    let set: MockSet = xs.iter().copied().collect();

    for &x in &xs {
        assert!(set.contains(x));
    }

    assert_eq!(set.iter().len(), xs.len() - 1);
}

#[test]
fn test_move_iter() {
    let hs = {
        let mut hs = MockSet::new(Key::MAX as usize);

        hs.insert_one(1);
        hs.insert_one(2);

        hs
    };

    let v = hs.iter().copied().collect::<Vec<Key>>();
    assert!(v == [1, 2] || v == [2, 1]);
}

#[test]
fn test_eq() {
    // These constants once happened to expose a bug in insert_one().
    // I'm keeping them around to prevent a regression.
    let mut s1 = MockSet::new(Key::MAX as usize);

    s1.insert_one(1);
    s1.insert_one(2);
    s1.insert_one(3);

    let mut s2 = MockSet::new(Key::MAX as usize);

    s2.insert_one(1);
    s2.insert_one(2);

    assert!(s1 != s2);

    s2.insert_one(3);

    assert_eq!(s1, s2);
}

#[test]
fn test_show() {
    let mut set = MockSet::new(Key::MAX as usize);
    let empty = MockSet::new(Key::MAX as usize);

    set.insert_one(1);
    set.insert_one(2);

    let set_str = format!("{set:?}");

    assert!(set_str == "{1, 2}" || set_str == "{2, 1}");
    assert_eq!(format!("{empty:?}"), "{}");
}

#[test]
#[allow(clippy::needless_borrow)]
fn test_extend_ref() {
    let mut a = MockSet::new(Key::MAX as usize);
    a.insert_one(1);

    a.extend([2, 3, 4]);

    assert_eq!(a.len(), 4);
    assert!(a.contains(1));
    assert!(a.contains(2));
    assert!(a.contains(3));
    assert!(a.contains(4));

    let mut b = MockSet::new(Key::MAX as usize);
    b.insert_one(5);
    b.insert_one(6);

    a.extend(b.iter());

    assert_eq!(a.len(), 6);
    assert!(a.contains(1));
    assert!(a.contains(2));
    assert!(a.contains(3));
    assert!(a.contains(4));
    assert!(a.contains(5));
    assert!(a.contains(6));
}

#[test]
fn test_retain() {
    const XS: [Key; 6] = [1, 2, 3, 4, 5, 6];
    let mut set: MockSet = MockSet::from_iter(XS);
    set.retain(|&k| k % 2 == 0);
    assert_eq!(set.len(), 3);
    assert!(set.contains(2));
    assert!(set.contains(4));
    assert!(set.contains(6));
}

#[test]
fn test_recall() {
    const XS: std::ops::Range<Key> = 0..8;
    {
        let mut set: MockSet = MockSet::from_iter(XS);
        let drained = set.recall(|&k| k % 2 == 0);
        let mut out = drained.collect::<Vec<_>>();
        out.sort_unstable();
        assert_eq!(vec![0, 2, 4, 6], out);
        assert_eq!([7, 1, 5, 3], set.as_slice());
        assert_eq!(set.len(), 4);
    }
    {
        let mut set: MockSet = MockSet::from_iter(XS);
        set.recall(|&k| k % 2 == 0).for_each(drop);
        assert_eq!(set.len(), 4, "Removes non-matching items on drop");
    }
}

#[test]
fn rehash_in_place() {
    let mut set = MockSet::new(Key::MAX as usize);

    for i in 0..224 {
        set.insert_one(i);
    }

    assert_eq!(
        set.len(),
        224,
        "The set must be at or close to capacity to trigger a re hashing"
    );

    for i in 100..1400 {
        set.delete_one(i - 100);
        set.insert_one(i);
    }
}

#[test]
fn collect() {
    // At the time of writing, this hits the ZST case in from_base_index
    // (and without the `map`, it does not).
    let mut _set: MockSet = (0..3).collect();
}

#[test]
fn duplicate_insert_one() {
    let mut set = MockSet::new(Key::MAX as usize);
    set.insert_one(1);
    set.insert_one(1);
    assert!([1].iter().eq(set.iter()));
}
