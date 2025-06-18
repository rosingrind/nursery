use itertools::Itertools;

use crate::{Area, BspaNode};

#[inline]
pub fn assert_node_expand(x: &BspaNode) {
    // blocks don't overlap
    debug_assert_eq!(
        x.blocks
            .iter()
            .tuple_combinations()
            .find(|(l, r)| { l.overlaps(r) }),
        None
    );
    // blocks and spaces don't overlap
    debug_assert_eq!(
        x.blocks.iter().find_map(|b| x
            .spaces
            .iter()
            .find_map(|s| s.overlaps(b).then_some((s, b)))),
        None
    );
    // avai_blk don't exceed avai_box limits
    debug_assert_eq!(
        x.avai_blk.iter().find(|b| {
            x.avai_box.iter().any(|(k, v)| {
                let d = b.list.iter().filter(|p| &p.item == k).count();
                v.checked_sub(d).is_none()
            })
        }),
        None
    );
    // avai_blk covers avai_box variations
    debug_assert_eq!(
        x.avai_box.iter().find(|&(k, v)| {
            !(0..=*v).skip(1).all(|n| {
                x.avai_blk
                    .iter()
                    .any(|b| b.list.iter().filter(|p| &p.item == k).count() == n)
            })
        }),
        None,
        "\navai_box: {:?}\navai_blk: {:?}",
        x.avai_box,
        x.avai_blk
    )
}

#[inline]
pub fn assert_node_inflate(x: &BspaNode) {
    // spaces aligned to max height
    debug_assert!(
        x.spaces
            .iter()
            .filter(|s| s.y + s.h() >= x.h())
            .map(|s| s.y + s.h())
            .all_equal(),
        "{:?}",
        x.spaces
    );
}
