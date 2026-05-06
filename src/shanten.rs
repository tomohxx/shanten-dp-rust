use crate::common::{NUM_TIDS, chmin};

#[derive(Debug, thiserror::Error)]
pub enum ShantenError {
    #[error("Invalid number of hand's tiles at {0}: {1}")]
    InvalidHand(usize, i8),
    #[error("Invalid number of tile_limits' at {0}: {1}")]
    InvalidTileLimits(usize, i8),
    #[error("Invalid sum of hands's melds: {0}")]
    InvalidMelds(usize),
}

pub fn calc_shanten(
    hand: &[i8; 34],
    tile_limits: &[i8; 35],
    m: usize,
    check_hand: bool,
) -> Result<i8, ShantenError> {
    if check_hand {
        for i in 0..NUM_TIDS {
            if hand[i] < 0 || hand[i] > 4 {
                return Err(ShantenError::InvalidHand(i, hand[i]));
            }

            if tile_limits[i] < 0 || tile_limits[i] > 4 || hand[i] > tile_limits[i] {
                return Err(ShantenError::InvalidTileLimits(i, tile_limits[i]));
            }
        }

        if m > 4 {
            return Err(ShantenError::InvalidMelds(m));
        }
    }

    let mut ret = super::standard::calc_shanten(hand, tile_limits, m);

    if m == 4 {
        chmin(&mut ret, super::seven_pairs::calc_shanten(hand, tile_limits));
        chmin(&mut ret, super::thirteen_orphans::calc_shanten(hand, tile_limits));
    }

    Ok(ret)
}

pub fn make_tile_limits(three_player: bool) -> [i8; 35] {
    let mut tile_limits = [4i8; 35];

    if three_player {
        tile_limits[1..8].fill(0);
    }

    tile_limits
}
