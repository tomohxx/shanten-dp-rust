use crate::common::{Calculatable, Data, MAX_SHT, NUM_TIDS};
use bitflags::{Flags, bitflags};

bitflags! {
    /// Calculation mode.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Mode: u8 {
        const STANDARD = 0b001;
        const SEVEN_PAIRS = 0b010;
        const THIRTEEN_ORPHANS = 0b100;
    }
}

/// Errors returned by [`calc_shanten`].
#[derive(Debug, thiserror::Error)]
pub enum ShantenError {
    /// The hand contains an invalid tile count.
    #[error("Invalid number of hand's tiles at {0}: {1}")]
    InvalidHand(usize, u8),
    /// The tile availability constraints contain an invalid tile count.
    #[error("Invalid number of tile_limits' at {0}: {1}")]
    InvalidTileLimits(usize, u8),
    /// The number of melds is outside the supported range.
    #[error("Invalid sum of hands's melds: {0}")]
    InvalidMelds(usize),
    #[error("Invalid calculation mode: {0:?}")]
    InvalidMode(Mode),
}

/// Calculates the shanten number for a hand.
///
/// # Arguments
///
/// * `hand` - Tile counts for the hand.
/// * `tile_limits` - Per-tile availability constraints.
/// * `m` - Number of hand tiles divided by 3.
/// * `mode` - Calculation mode ([`Mode`]). Combine flags with bitwise OR (e.g., `Mode::STANDARD | Mode::SEVEN_PAIRS`) or use `Mode::all()`.
/// * `check_hand` - Validates the arguments when set to `true`.
///
/// # Errors
///
/// Returns [`Err`] if any argument is invalid.
///
/// # Examples
///
/// ```
/// # use shanten_dp::{ShantenError, calc_shanten, make_tile_limits, Mode};
/// # fn main() -> Result<(), ShantenError> {
/// let hand: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
///     0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
///     1, 0, 1, 0, 3, 0, 0, // jihai
/// ];
/// let tile_limits = make_tile_limits(false);
/// let shanten = calc_shanten(&hand, &tile_limits, 4, Mode::all(), true)?;
///
/// assert!(matches!(shanten, Some(2)));
/// # Ok(())
/// # }
/// ```
pub fn calc_shanten(
    hand: &[u8; 34],
    tile_limits: &[u8; 35],
    m: usize,
    mode: Mode,
    check_hand: bool,
) -> Result<Option<i8>, ShantenError> {
    calc_shanten_impl::<i8>(hand, tile_limits, m, mode, check_hand)
}

/// Calculates the shanten number for a hand, along with its necessary tiles / missing tiles and unnecessary tiles / redundant tiles.
///
/// # Arguments
///
/// * `hand` - Tile counts for the hand.
/// * `tile_limits` - Per-tile availability constraints.
/// * `m` - Number of hand tiles divided by 3.
/// * `mode` - Calculation mode ([`Mode`]). Combine flags with bitwise OR (e.g., `Mode::STANDARD | Mode::SEVEN_PAIRS`) or use `Mode::all()`.
/// * `check_hand` - Validates the arguments when set to `true`.
///
/// # Errors
///
/// Returns [`Err`] if any argument is invalid.
///
/// # Examples
///
/// ```
/// # use shanten_dp::{Data, ShantenError, calc_shanten2, make_tile_limits, Mode};
/// # fn main() -> Result<(), ShantenError> {
/// let hand: [u8; 34] = [
///     1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
///     0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
///     0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
///     1, 0, 1, 0, 3, 0, 0, // jihai
/// ];
/// let tile_limits = make_tile_limits(false);
/// let Data { shanten, discards, waits } =
///     calc_shanten2(&hand, &tile_limits, 4, Mode::all(), true)?.unwrap();
///
/// assert_eq!(shanten, 2);
/// assert_eq!(discards, 0b0010101_000000000_101011010_000000000);
/// assert_eq!(waits, 0b0000101_000000000_111111111_000000000);
/// # Ok(())
/// # }
/// ```
pub fn calc_shanten2(
    hand: &[u8; 34],
    tile_limits: &[u8; 35],
    m: usize,
    mode: Mode,
    check_hand: bool,
) -> Result<Option<Data>, ShantenError> {
    calc_shanten_impl::<Data>(hand, tile_limits, m, mode, check_hand)
}

pub fn calc_shanten_impl<T: Calculatable>(
    hand: &[u8; 34],
    tile_limits: &[u8; 35],
    m: usize,
    mode: Mode,
    check_hand: bool,
) -> Result<Option<T>, ShantenError> {
    if check_hand {
        for i in 0..NUM_TIDS {
            if hand[i] > 4 {
                return Err(ShantenError::InvalidHand(i, hand[i]));
            }

            if tile_limits[i] > 4 || hand[i] > tile_limits[i] {
                return Err(ShantenError::InvalidTileLimits(i, tile_limits[i]));
            }
        }

        if m > 4 {
            return Err(ShantenError::InvalidMelds(m));
        }

        if mode.is_empty() || mode.contains_unknown_bits() {
            return Err(ShantenError::InvalidMode(mode));
        }
    }

    let mut ret = T::new(MAX_SHT);

    if mode.contains(Mode::STANDARD) {
        ret.chmin(super::standard::calc_shanten::<T>(hand, tile_limits, m));
    }

    if m == 4 {
        if mode.contains(Mode::SEVEN_PAIRS) {
            ret.chmin(super::seven_pairs::calc_shanten::<T>(hand, tile_limits));
        }

        if mode.contains(Mode::THIRTEEN_ORPHANS) {
            ret.chmin(super::thirteen_orphans::calc_shanten::<T>(hand, tile_limits));
        }
    }

    Ok(if ret == MAX_SHT { None } else { Some(ret) })
}

/// Creates tile availability constraints.
///
/// # Arguments
///
/// * `three_player` - If `false`, sets the available counts for all tiles to `4`.
///   If `true`, sets the available counts for *2m* through *8m* to `0` and all other
///   tiles to `4`.
pub const fn make_tile_limits(three_player: bool) -> [u8; 35] {
    let mut tile_limits = [4u8; 35];

    if three_player {
        tile_limits[1] = 0; // 2m
        tile_limits[2] = 0; // 3m
        tile_limits[3] = 0; // 4m
        tile_limits[4] = 0; // 5m
        tile_limits[5] = 0; // 6m
        tile_limits[6] = 0; // 7m
        tile_limits[7] = 0; // 8m
    }

    tile_limits
}
