use crate::common::*;

pub fn calc_shanten<T: Calculatable>(hand: &[u8; 34], tile_limits: &[u8; 35]) -> T {
    let mut table = [[T::new(MAX_SHT); 8]; 35];

    table[0][0] = T::new(-1);

    for n in 0..NUM_TIDS {
        for pp in 0..(tile_limits[n] / 2 + 1).min(2) as usize {
            for p in 0..8usize.saturating_sub(pp) {
                let current = table[n][p];

                if current == MAX_SHT {
                    continue;
                }

                let distance = 2 * pp as i8 - hand[n] as i8;

                table[n + 1][p + pp].chmin(current.get_next_value(distance, n));
            }
        }
    }

    table[34][7]
}
