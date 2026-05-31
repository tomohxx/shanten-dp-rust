use crate::common::*;

const NON_SIMPLES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];

pub fn calc_shanten(hand: &[u8; 34], tile_limits: &[u8; 35]) -> i8 {
    let mut table = [[MAX_SHT; 2]; 14];

    table[0][0] = 0;

    for (n, tid) in NON_SIMPLES.iter().enumerate() {
        for pp in 0..(tile_limits[*tid]).min(2) as usize {
            for p in 0..2usize.saturating_sub(pp) {
                let current = table[n][p];

                if current == MAX_SHT {
                    continue;
                }

                let distance = (pp + 1).saturating_sub(hand[*tid] as usize) as u8;

                chmin(&mut table[n + 1][p + pp], current + distance);
            }
        }
    }

    table[13][1] as i8 - 1
}
