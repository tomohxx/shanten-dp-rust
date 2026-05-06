use crate::common::{MAX_SHT, chmin, get_next_value};

const NON_SIMPLES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];

pub fn calc_shanten(hand: &[i8; 34], tile_limits: &[i8; 35]) -> i8 {
    let mut table = [[MAX_SHT; 2]; 14];

    table[0][0] = 0;

    for n in 0..13 {
        for p in 0..=1 {
            let current = table[n][p];

            if current == MAX_SHT {
                continue;
            }

            for pp in 0..=1usize {
                if pp as i8 + 1 > tile_limits[NON_SIMPLES[n]] || p + pp > 1 {
                    break;
                }

                let distance = pp as i8 + 1 - hand[NON_SIMPLES[n]];

                chmin(&mut table[n + 1][p + pp], get_next_value(current, distance));
            }
        }
    }

    table[13][1] - 1
}
