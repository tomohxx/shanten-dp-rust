use crate::common::*;

struct Delta {
    a: usize,
    b: usize,
    c: usize,
    h: usize,
    m: usize,
}

const DELTAS_WITH_SEQ: [Delta; 8] = [
    Delta { a: 0, b: 0, c: 0, h: 0, m: 0 },
    Delta { a: 1, b: 1, c: 1, h: 0, m: 1 },
    Delta { a: 2, b: 2, c: 2, h: 0, m: 2 },
    Delta { a: 3, b: 0, c: 0, h: 0, m: 1 },
    Delta { a: 4, b: 1, c: 1, h: 0, m: 2 },
    Delta { a: 2, b: 0, c: 0, h: 1, m: 0 },
    Delta { a: 3, b: 1, c: 1, h: 1, m: 1 },
    Delta { a: 4, b: 2, c: 2, h: 1, m: 2 },
];

const DELTAS_WITHOUT_SEQ: [Delta; 3] = [
    Delta { a: 0, b: 0, c: 0, h: 0, m: 0 },
    Delta { a: 3, b: 0, c: 0, h: 0, m: 1 },
    Delta { a: 2, b: 0, c: 0, h: 1, m: 0 },
];

const DELTAS: [&[Delta]; 34] = [
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITH_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
    &DELTAS_WITHOUT_SEQ,
];

pub fn calc_shanten(hand: &[i8; 34], tile_limits: &[i8; 35], m: usize) -> i8 {
    let mut table = [[[[[MAX_SHT; 5]; 2]; 5]; 5]; 35];

    table[0][0][0][0][0] = 0;

    for n in 0..NUM_TIDS {
        for delta in DELTAS[n] {
            for a in 0..(tile_limits[n] as usize + 1).saturating_sub(delta.a) {
                for b in 0..(tile_limits[n + 1] as usize + 1).saturating_sub(delta.b).min(a + 1) {
                    for h in 0..2usize.saturating_sub(delta.h) {
                        for mm in 0..(m + 1).saturating_sub(delta.m) {
                            let current = table[n][a][b][h][mm];

                            if current == MAX_SHT {
                                continue;
                            }

                            let distance = (a + delta.a).saturating_sub(hand[n] as usize) as u8;

                            chmin(
                                &mut table[n + 1][b + delta.b][delta.c][h + delta.h][mm + delta.m],
                                current + distance,
                            );
                        }
                    }
                }
            }
        }
    }

    table[NUM_TIDS][0][0][1][m] as i8 - 1
}
