use beamsrch::Node;

use crate::{Area, BspaNode, types::*};

const SIDE: u32 = 8;
const PADD: u32 = 4;

#[test]
fn basic_beam_functions() {
    const RECT_0: Rect = Rect::new(SIDE, SIDE);
    const RECT_1: Rect = Rect::new(SIDE, PADD);
    const SPACES: [Placement<Rect>; 3] = [
        Placement {
            x: 0,
            y: SIDE,
            item: Rect::new(SIDE * 3, PADD),
        },
        Placement {
            x: SIDE,
            y: PADD,
            item: Rect::new(SIDE, PADD * 2),
        },
        Placement {
            x: SIDE * 3,
            y: 0,
            item: Rect::new(PADD, SIDE + PADD),
        },
    ];

    let tmp = BspaNode::new([RECT_0, RECT_1].repeat(5), SIDE * 3, 0, 1.0);
    let mut node = BspaNode {
        spaces: SPACES.to_vec(),
        blocks: vec![Placement {
            x: 0,
            y: 0,
            item: [
                Placement {
                    x: 0,
                    y: 0,
                    item: RECT_0,
                },
                Placement {
                    x: SIDE,
                    y: 0,
                    item: RECT_1,
                },
                Placement {
                    x: SIDE * 2,
                    y: 0,
                    item: RECT_0,
                },
            ]
            .into_iter()
            .collect(),
        }],
        avai_box: tmp.avai_box,
        avai_blk: tmp.avai_blk,
    };

    Node::<0>::inflate(&mut node);

    let xmax = node
        .spaces
        .iter()
        .map(|s| s.x + s.w())
        .max()
        .unwrap_or_else(|| node.w());
    assert_eq!(
        node.spaces,
        vec![
            Placement {
                x: SPACES[0].x,
                y: SPACES[0].y,
                item: Rect::new(SPACES[0].item.w(), 44 - SPACES[0].y)
            },
            Placement {
                x: SPACES[1].x,
                y: SPACES[1].y,
                item: Rect::new(SPACES[1].item.w(), 44 - SPACES[1].y)
            },
            Placement {
                x: SPACES[2].x,
                y: SPACES[2].y,
                item: Rect::new(SPACES[2].item.w(), 44 - SPACES[2].y),
            },
            Placement {
                x: 0,
                y: node.h(),
                item: Rect::new(xmax, 44 - node.h()),
            },
        ]
    );
}
