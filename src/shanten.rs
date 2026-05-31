use crate::common::{MAX_SHT, NUM_TIDS};

/// Errors returned by [`calc_shanten`].
#[derive(Debug, thiserror::Error)]
pub enum ShantenError {
    /// The hand contains an invalid tile count.
    #[error("Invalid number of hand's tiles at {0}: {1}")]
    InvalidHand(usize, i8),
    /// The tile availability constraints contain an invalid tile count.
    #[error("Invalid number of tile_limits' at {0}: {1}")]
    InvalidTileLimits(usize, i8),
    /// The number of melds is outside the supported range.
    #[error("Invalid sum of hands's melds: {0}")]
    InvalidMelds(usize),
    #[error("Invalid calculation mode: {0}")]
    InvalidMode(u8),
}

/// Calculates the shanten number for a hand.
///
/// # Arguments
///
/// * `hand` - Tile counts for the hand.
/// * `tile_limits` - Per-tile availability constraints.
/// * `m` - Number of hand tiles divided by 3.
/// * `mode` - Bit flags for the hand types to calculate: 1 for standard, 2 for seven pairs, and 4 for thirteen orphans. Use bitwise OR to combine them.
/// * `check_hand` - Validates the arguments when set to `true`.
///
/// # Errors
///
/// Returns [`Err`] if any argument is invalid.
///
/// # Examples
///
/// ```
/// # use shanten_dp::{ShantenError, calc_shanten, make_tile_limits};
/// # fn main() -> Result<(), ShantenError> {
/// let hand: [i8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
///     0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
///     1, 0, 1, 0, 3, 0, 0, // jihai
/// ];
/// let tile_limits = make_tile_limits(false);
/// let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true)?;
///
/// assert!(matches!(shanten, Some(2)));
/// # Ok(())
/// # }
/// ```
pub fn calc_shanten(
    hand: &[i8; 34],
    tile_limits: &[i8; 35],
    m: usize,
    mode: u8,
    check_hand: bool,
) -> Result<Option<i8>, ShantenError> {
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

        if mode == 0 || mode > 7 || (m != 4 && mode & 1 == 0) {
            return Err(ShantenError::InvalidMode(mode));
        }
    }

    let mut ret = MAX_SHT as i8;

    if mode & 1 != 0 {
        ret = ret.min(super::standard::calc_shanten(hand, tile_limits, m));
    }

    if m == 4 {
        if mode & 2 != 0 {
            ret = ret.min(super::seven_pairs::calc_shanten(hand, tile_limits));
        }

        if mode & 4 != 0 {
            ret = ret.min(super::thirteen_orphans::calc_shanten(hand, tile_limits));
        }
    }

    Ok(if ret == MAX_SHT as i8 - 1 { None } else { Some(ret) })
}

/// Creates tile availability constraints.
///
/// # Arguments
///
/// * `three_player` - If `false`, sets the available counts for all tiles to `4`.
///   If `true`, sets the available counts for *2m* through *8m* to `0` and all other
///   tiles to `4`.
pub fn make_tile_limits(three_player: bool) -> [i8; 35] {
    let mut tile_limits = [4i8; 35];

    if three_player {
        tile_limits[1..8].fill(0);
    }

    tile_limits
}
