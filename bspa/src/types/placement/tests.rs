use itertools::Itertools;

use super::*;

const SIDE: u32 = 8;
const PADD: u32 = 4;

#[test]
fn basic_split_functions() {
    const LHS: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };

    assert_eq!(LHS.substract(&LHS).collect_array::<0>(), Some([]));

    const B1: Placement<Rect> = Placement {
        x: 0,
        y: 15,
        item: Rect::new(32, 43),
    };
    const B2: Placement<Rect> = Placement {
        x: 15,
        y: 0,
        item: Rect::new(12, 16),
    };
    assert!(B1.overlaps(&B2));

    const B3: Placement<Rect> = Placement {
        x: 16,
        y: 0,
        item: Rect::new(16, 58),
    };
    const B4: Placement<Rect> = Placement {
        x: 16,
        y: 0,
        item: Rect::new(14, 14),
    };
    assert!(B3.overlaps(&B4));
}

#[test]
fn basic_split_in_corner() {
    const LHS: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_N: Placement<Rect> = Placement {
        x: 0,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_S: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(SIDE, PADD),
    };
    const RHS_E: Placement<Rect> = Placement {
        x: PADD,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_W: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(PADD, SIDE),
    };

    assert_eq!(
        LHS.split_n(&RHS_N),
        Some(Placement {
            x: 0,
            y: 0,
            item: Rect::new(SIDE, PADD),
        })
    );

    assert_eq!(
        LHS.split_s(&RHS_S),
        Some(Placement {
            x: 0,
            y: PADD,
            item: Rect::new(SIDE, PADD),
        })
    );

    assert_eq!(
        LHS.split_e(&RHS_E),
        Some(Placement {
            x: 0,
            y: 0,
            item: Rect::new(PADD, SIDE),
        })
    );

    assert_eq!(
        LHS.split_w(&RHS_W),
        Some(Placement {
            x: PADD,
            y: 0,
            item: Rect::new(PADD, SIDE),
        })
    );

    let lhs = [RHS_N, RHS_S, RHS_E, RHS_W]
        .iter()
        .fold(vec![LHS], |mut acc, rhs| {
            acc = acc.drain(..).flat_map(|lhs| lhs.substract(rhs)).collect();
            acc
        });
    assert_eq!(lhs, Vec::new());
}

#[test]
fn basic_split_in_center() {
    const LHS: Placement<Rect> = Placement {
        x: PADD,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_N: Placement<Rect> = Placement {
        x: PADD,
        y: SIDE,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_S: Placement<Rect> = Placement {
        x: PADD,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_E: Placement<Rect> = Placement {
        x: SIDE,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_W: Placement<Rect> = Placement {
        x: 0,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };

    assert_eq!(
        LHS.split_n(&RHS_N),
        Some(Placement {
            x: PADD,
            y: PADD,
            item: Rect::new(SIDE, PADD),
        })
    );

    assert_eq!(
        LHS.split_s(&RHS_S),
        Some(Placement {
            x: PADD,
            y: SIDE,
            item: Rect::new(SIDE, PADD),
        })
    );

    assert_eq!(
        LHS.split_e(&RHS_E),
        Some(Placement {
            x: PADD,
            y: PADD,
            item: Rect::new(PADD, SIDE),
        })
    );

    assert_eq!(
        LHS.split_w(&RHS_W),
        Some(Placement {
            x: SIDE,
            y: PADD,
            item: Rect::new(PADD, SIDE),
        })
    );

    let lhs = [RHS_N, RHS_S, RHS_E, RHS_W]
        .iter()
        .fold(vec![LHS], |mut acc, rhs| {
            acc = acc.drain(..).flat_map(|lhs| lhs.substract(rhs)).collect();
            acc
        });
    assert_eq!(lhs, Vec::new());
}

#[test]
fn inter_split_in_corner() {
    const LHS: Placement<Rect> = Placement {
        x: PADD,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_NE: Placement<Rect> = Placement {
        x: SIDE,
        y: SIDE,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_NW: Placement<Rect> = Placement {
        x: 0,
        y: SIDE,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_SE: Placement<Rect> = Placement {
        x: SIDE,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_SW: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(SIDE, SIDE),
    };

    assert_eq!(
        LHS.substract(&RHS_NE).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(SIDE, PADD),
            },
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(PADD, SIDE),
            }
        ])
    );

    assert_eq!(
        LHS.substract(&RHS_NW).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(SIDE, PADD),
            },
            Placement {
                x: SIDE,
                y: PADD,
                item: Rect::new(PADD, SIDE),
            }
        ])
    );

    assert_eq!(
        LHS.substract(&RHS_SE).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: SIDE,
                item: Rect::new(SIDE, PADD),
            },
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(PADD, SIDE),
            }
        ])
    );

    assert_eq!(
        LHS.substract(&RHS_SW).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: SIDE,
                item: Rect::new(SIDE, PADD),
            },
            Placement {
                x: SIDE,
                y: PADD,
                item: Rect::new(PADD, SIDE),
            }
        ])
    );
}

#[test]
fn inter_split_in_center() {
    const LHS: Placement<Rect> = Placement {
        x: PADD,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_LAT: Placement<Rect> = Placement {
        x: PADD,
        y: PADD + SIDE / 4,
        item: Rect::new(SIDE, PADD),
    };
    const RHS_LON: Placement<Rect> = Placement {
        x: PADD + SIDE / 4,
        y: PADD,
        item: Rect::new(PADD, SIDE),
    };
    const RHS_MID: Placement<Rect> = Placement {
        x: PADD + SIDE / 4,
        y: PADD + SIDE / 4,
        item: Rect::new(PADD, PADD),
    };

    assert_eq!(
        LHS.substract(&RHS_LAT).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(SIDE, SIDE / 4),
            },
            Placement {
                x: PADD,
                y: PADD + SIDE / 4 * 3,
                item: Rect::new(SIDE, SIDE / 4),
            }
        ])
    );

    assert_eq!(
        LHS.substract(&RHS_LON).collect_array::<2>(),
        Some([
            Placement {
                x: PADD,
                y: PADD,
                item: Rect::new(SIDE / 4, SIDE),
            },
            Placement {
                x: PADD + SIDE / 4 * 3,
                y: PADD,
                item: Rect::new(SIDE / 4, SIDE),
            }
        ])
    );

    assert_eq!(
        LHS.substract(&RHS_MID).collect_array::<4>(),
        [
            LHS.substract(&RHS_LAT).collect_array::<2>().unwrap(),
            LHS.substract(&RHS_LON).collect_array::<2>().unwrap()
        ]
        .as_flattened()
        .try_into()
        .ok()
    );
}

#[test]
fn bound_avoid_in_center() {
    const LHS: Placement<Rect> = Placement {
        x: PADD,
        y: PADD,
        item: Rect::new(SIDE, SIDE),
    };
    const RHS_N: Placement<Rect> = Placement {
        x: 0,
        y: PADD + SIDE + 1,
        item: Rect::new(PADD * 2 + SIDE, PADD - 1),
    };
    const RHS_S: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(PADD * 2 + SIDE, PADD - 1),
    };
    const RHS_E: Placement<Rect> = Placement {
        x: PADD + SIDE + 1,
        y: 0,
        item: Rect::new(PADD - 1, PADD * 2 + SIDE),
    };
    const RHS_W: Placement<Rect> = Placement {
        x: 0,
        y: 0,
        item: Rect::new(PADD - 1, PADD * 2 + SIDE),
    };

    assert_eq!(
        LHS.overlaps(&RHS_N)
            .then_some(LHS.split_n(&RHS_N))
            .flatten(),
        None
    );

    assert_eq!(
        LHS.overlaps(&RHS_S)
            .then_some(LHS.split_s(&RHS_S))
            .flatten(),
        None
    );

    assert_eq!(
        LHS.overlaps(&RHS_E)
            .then_some(LHS.split_e(&RHS_E))
            .flatten(),
        None
    );

    assert_eq!(
        LHS.overlaps(&RHS_W)
            .then_some(LHS.split_w(&RHS_W))
            .flatten(),
        None
    );

    let lhs = {
        let mut arr = [RHS_N, RHS_S, RHS_E, RHS_W];
        (0..arr.len()).fold(Vec::with_capacity(arr.len()), |mut acc, i| {
            arr.rotate_left(i);
            let item = arr.first().unwrap();
            acc.extend(
                arr.iter()
                    .skip(1)
                    .filter(|&x| x.overlaps(item))
                    .flat_map(|x| item.substract(x)),
            );
            acc
        })
    };
    assert_eq!(
        lhs,
        vec![
            Placement {
                x: RHS_N.x,
                y: RHS_N.y,
                item: Rect::new(RHS_N.w() - (PADD - 1), RHS_N.h()),
            },
            Placement {
                x: RHS_N.x + (PADD - 1),
                y: RHS_N.y,
                item: Rect::new(RHS_N.w() - (PADD - 1), RHS_N.h()),
            },
            Placement {
                x: RHS_S.x,
                y: RHS_S.y,
                item: Rect::new(RHS_S.w() - (PADD - 1), RHS_S.h()),
            },
            Placement {
                x: RHS_S.x + (PADD - 1),
                y: RHS_S.y,
                item: Rect::new(RHS_S.w() - (PADD - 1), RHS_S.h()),
            },
            Placement {
                x: RHS_W.x,
                y: RHS_W.y,
                item: Rect::new(RHS_W.w(), RHS_W.h() - (PADD - 1)),
            },
            Placement {
                x: RHS_W.x,
                y: RHS_W.y + (PADD - 1),
                item: Rect::new(RHS_W.w(), RHS_W.h() - (PADD - 1)),
            },
            Placement {
                x: RHS_E.x,
                y: RHS_E.y,
                item: Rect::new(RHS_E.w(), RHS_E.h() - (PADD - 1)),
            },
            Placement {
                x: RHS_E.x,
                y: RHS_E.y + (PADD - 1),
                item: Rect::new(RHS_E.w(), RHS_E.h() - (PADD - 1)),
            }
        ]
    );
}
