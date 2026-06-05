use crate::common::*;

const NON_SIMPLES: [usize; 13] = [0, 8, 9, 17, 18, 26, 27, 28, 29, 30, 31, 32, 33];

pub fn calc_shanten<T: Calculatable>(hand: &[u8; NUM_TIDS], tile_limits: &[u8; NUM_TIDS + 1]) -> T {
    let mut table = [[T::new(MAX_SHT); 2]; 14];

    table[0][0] = T::new(-1);

    for (n, tid) in NON_SIMPLES.iter().enumerate() {
        for pp in 0..(tile_limits[*tid]).min(2) as usize {
            for p in 0..2usize.saturating_sub(pp) {
                let current = table[n][p];

                if current == MAX_SHT {
                    continue;
                }

                let distance = pp as i8 + 1 - hand[*tid] as i8;

                table[n + 1][p + pp].chmin(current.get_next_value(distance, *tid));
            }
        }
    }

    table[NON_SIMPLES.len()][1]
}
