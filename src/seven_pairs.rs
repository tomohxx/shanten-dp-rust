use crate::common::{MAX_SHT, NUM_TIDS, chmin, get_next_value};

pub fn calc_shanten(hand: &[i8; 34], tile_limits: &[i8; 35]) -> i8 {
    let mut table = [[MAX_SHT; 8]; 35];

    table[0][0] = -1;

    for n in 0..NUM_TIDS {
        for pp in 0..(tile_limits[n] / 2 + 1).min(2) as usize {
            for p in 0..8usize.saturating_sub(pp) {
                let current = table[n][p];

                if current == MAX_SHT {
                    continue;
                }

                let distance = 2 * pp as i8 - hand[n];

                chmin(&mut table[n + 1][p + pp], get_next_value(current, distance));
            }
        }
    }

    table[34][7]
}
