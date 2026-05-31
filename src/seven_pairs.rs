use crate::common::*;

pub fn calc_shanten(hand: &[u8; 34], tile_limits: &[u8; 35]) -> i8 {
    let mut table = [[MAX_SHT; 8]; 35];

    table[0][0] = 0;

    for n in 0..NUM_TIDS {
        for pp in 0..(tile_limits[n] / 2 + 1).min(2) as usize {
            for p in 0..8usize.saturating_sub(pp) {
                let current = table[n][p];

                if current == MAX_SHT {
                    continue;
                }

                let distance = (2 * pp).saturating_sub(hand[n] as usize) as u8;

                chmin(&mut table[n + 1][p + pp], current + distance);
            }
        }
    }

    table[34][7] as i8 - 1
}
