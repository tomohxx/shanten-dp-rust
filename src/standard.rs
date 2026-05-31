use crate::common::*;

struct Delta {
    a: usize,
    b: usize,
    c: usize,
    p: usize,
    m: usize,
}

const DELTAS_WITH_SEQ: [Delta; 8] = [
    Delta { a: 0, b: 0, c: 0, p: 0, m: 0 },
    Delta { a: 1, b: 1, c: 1, p: 0, m: 1 },
    Delta { a: 2, b: 2, c: 2, p: 0, m: 2 },
    Delta { a: 3, b: 0, c: 0, p: 0, m: 1 },
    Delta { a: 4, b: 1, c: 1, p: 0, m: 2 },
    Delta { a: 2, b: 0, c: 0, p: 1, m: 0 },
    Delta { a: 3, b: 1, c: 1, p: 1, m: 1 },
    Delta { a: 4, b: 2, c: 2, p: 1, m: 2 },
];

const DELTAS_WITHOUT_SEQ: [Delta; 3] = [
    Delta { a: 0, b: 0, c: 0, p: 0, m: 0 },
    Delta { a: 3, b: 0, c: 0, p: 0, m: 1 },
    Delta { a: 2, b: 0, c: 0, p: 1, m: 0 },
];

const DELTAS: [&[Delta]; 34] = [
    &DELTAS_WITH_SEQ,    // 1m
    &DELTAS_WITH_SEQ,    // 2m
    &DELTAS_WITH_SEQ,    // 3m
    &DELTAS_WITH_SEQ,    // 4m
    &DELTAS_WITH_SEQ,    // 5m
    &DELTAS_WITH_SEQ,    // 6m
    &DELTAS_WITH_SEQ,    // 7m
    &DELTAS_WITHOUT_SEQ, // 8m
    &DELTAS_WITHOUT_SEQ, // 9m
    &DELTAS_WITH_SEQ,    // 1p
    &DELTAS_WITH_SEQ,    // 2p
    &DELTAS_WITH_SEQ,    // 3p
    &DELTAS_WITH_SEQ,    // 4p
    &DELTAS_WITH_SEQ,    // 5p
    &DELTAS_WITH_SEQ,    // 6p
    &DELTAS_WITH_SEQ,    // 7p
    &DELTAS_WITHOUT_SEQ, // 8p
    &DELTAS_WITHOUT_SEQ, // 9p
    &DELTAS_WITH_SEQ,    // 1s
    &DELTAS_WITH_SEQ,    // 2s
    &DELTAS_WITH_SEQ,    // 3s
    &DELTAS_WITH_SEQ,    // 4s
    &DELTAS_WITH_SEQ,    // 5s
    &DELTAS_WITH_SEQ,    // 6s
    &DELTAS_WITH_SEQ,    // 7s
    &DELTAS_WITHOUT_SEQ, // 8s
    &DELTAS_WITHOUT_SEQ, // 9s
    &DELTAS_WITHOUT_SEQ, // 1z
    &DELTAS_WITHOUT_SEQ, // 2z
    &DELTAS_WITHOUT_SEQ, // 3z
    &DELTAS_WITHOUT_SEQ, // 4z
    &DELTAS_WITHOUT_SEQ, // 5z
    &DELTAS_WITHOUT_SEQ, // 6z
    &DELTAS_WITHOUT_SEQ, // 7z
];

pub fn calc_shanten<T: Calculatable>(hand: &[u8; 34], tile_limits: &[u8; 35], m: usize) -> T {
    let mut table = [[[[[T::new(MAX_SHT); 5]; 2]; 5]; 5]; 35];

    table[0][0][0][0][0] = T::new(-1);

    for n in 0..NUM_TIDS {
        for delta in DELTAS[n] {
            for a in 0..(tile_limits[n] as usize + 1).saturating_sub(delta.a) {
                for b in 0..(tile_limits[n + 1] as usize + 1).saturating_sub(delta.b).min(a + 1) {
                    for p in 0..2usize.saturating_sub(delta.p) {
                        for mm in 0..(m + 1).saturating_sub(delta.m) {
                            let current = table[n][a][b][p][mm];

                            if current == MAX_SHT {
                                continue;
                            }

                            let distance = a as i8 + delta.a as i8 - hand[n] as i8;

                            table[n + 1][b + delta.b][delta.c][p + delta.p][mm + delta.m]
                                .chmin(current.get_next_value(distance, n));
                        }
                    }
                }
            }
        }
    }

    table[NUM_TIDS][0][0][1][m]
}
