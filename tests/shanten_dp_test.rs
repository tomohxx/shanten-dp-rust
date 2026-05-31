#![allow(dead_code)]
use shanten_dp::{Data, ShantenError, calc_shanten, calc_shanten2, make_tile_limits};

const M1: usize = 0;
const M2: usize = 1;
const M3: usize = 2;
const M4: usize = 3;
const M5: usize = 4;
const M6: usize = 5;
const M7: usize = 6;
const M8: usize = 7;
const M9: usize = 8;
const P1: usize = 9;
const P2: usize = 10;
const P3: usize = 11;
const P4: usize = 12;
const P5: usize = 13;
const P6: usize = 14;
const P7: usize = 15;
const P8: usize = 16;
const P9: usize = 17;
const S1: usize = 18;
const S2: usize = 19;
const S3: usize = 20;
const S4: usize = 21;
const S5: usize = 22;
const S6: usize = 23;
const S7: usize = 24;
const S8: usize = 25;
const S9: usize = 26;
const Z1: usize = 27;
const Z2: usize = 28;
const Z3: usize = 29;
const Z4: usize = 30;
const Z5: usize = 31;
const Z6: usize = 32;
const Z7: usize = 33;
#[test]
fn test_invalid_hand_error() {
    let hand: [u8; 34] = [
        5, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        0, 0, 0, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let err = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap_err();

    assert!(matches!(err, ShantenError::InvalidHand(0, 5)));
}

#[test]
fn test_invalid_tile_limits_error() {
    let hand: [u8; 34] = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        0, 0, 0, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = [0u8; 35];
    let err = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap_err();

    assert!(matches!(err, ShantenError::InvalidTileLimits(0, 0)));
}

#[test]
fn test_invalid_melds_error() {
    let hand: [u8; 34] = [0; 34];
    let tile_limits = make_tile_limits(false);
    let err = calc_shanten(&hand, &tile_limits, 5, 7, true).unwrap_err();

    assert!(matches!(err, ShantenError::InvalidMelds(5)));
}

#[test]
fn test_invalid_mode_error() {
    let hand: [u8; 34] = [0; 34];
    let tile_limits = make_tile_limits(false);
    let err = calc_shanten(&hand, &tile_limits, 4, 8, true).unwrap_err();

    assert!(matches!(err, ShantenError::InvalidMode(8)));
}

#[test]
fn test_closed_hand() {
    // 123m245779p13555z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        1, 0, 1, 0, 3, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_cannot_win() {
    // 123m245779p13555z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        1, 0, 1, 0, 3, 0, 0, // jihai
    ];
    let mut tile_limits = [0u8; 35];

    tile_limits[..hand.len()].copy_from_slice(&hand);

    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, None));
}

#[test]
fn test_insufficient_blocks_4433() {
    // 11112222333444z
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 3, 3, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(1)));
}

#[test]
fn test_insufficient_blocks_4442i() {
    // 11m111122223333z
    let hand: [u8; 34] = [
        2, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 4, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_insufficient_blocks_4442ii() {
    // 23m111122223333z
    let hand: [u8; 34] = [
        0, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 4, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_insufficient_blocks_4333() {
    // 1111222333444z
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 3, 3, 3, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(1)));
}

#[test]
fn test_insufficient_blocks_4432i() {
    // 11m11112222333z
    let hand: [u8; 34] = [
        2, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 3, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_insufficient_blocks_4432ii() {
    // 23m11112222333z
    let hand: [u8; 34] = [
        0, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 3, 0, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_insufficient_blocks_4441() {
    // 1111222233334z
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        4, 4, 4, 1, 0, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(3)));
}

#[test]
fn test_open_hand_1() {
    // 13m123456p11s[[2222m]]
    let hand: [u8; 34] = [
        1, 0, 1, 0, 0, 0, 0, 0, 0, // manzu
        1, 1, 1, 1, 1, 1, 0, 0, 0, // pinzu
        2, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        0, 0, 0, 0, 0, 0, 0, // jihai
    ];
    let mut tile_limits = make_tile_limits(false);

    tile_limits[M2] = 0;

    let shanten = calc_shanten(&hand, &tile_limits, 3, 7, true).unwrap();

    assert!(matches!(shanten, Some(1)));
}

#[test]
fn test_open_hand_2() {
    // 9m9p569s[999m][999p][999s]
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 1, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 1, // pinzu
        0, 0, 0, 0, 1, 1, 0, 0, 1, // souzu
        0, 0, 0, 0, 0, 0, 0, // jihai
    ];
    let mut tile_limits = make_tile_limits(false);

    tile_limits[M9] = 1;
    tile_limits[P9] = 1;
    tile_limits[S9] = 1;

    let shanten = calc_shanten(&hand, &tile_limits, 1, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_open_hand_3() {
    // 11567z[777z][666z][555z]
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        2, 0, 0, 0, 1, 1, 1, // jihai
    ];
    let mut tile_limits = make_tile_limits(false);

    tile_limits[Z7] = 1;
    tile_limits[Z6] = 1;
    tile_limits[Z5] = 1;

    let shanten = calc_shanten(&hand, &tile_limits, 1, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_open_hand_4() {
    // 13556z[111z][333z][666z]
    let hand: [u8; 34] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, // manzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        1, 0, 1, 0, 2, 1, 0, // jihai
    ];
    let mut tile_limits = make_tile_limits(false);

    tile_limits[Z1] = 1;
    tile_limits[Z3] = 1;
    tile_limits[Z6] = 1;

    let shanten = calc_shanten(&hand, &tile_limits, 1, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}

#[test]
fn test_discards_and_waits() {
    // 123m245779p13555z
    let hand: [u8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        1, 0, 1, 0, 3, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let Data { shanten, discards, waits } =
        calc_shanten2(&hand, &tile_limits, 4, 7, true).unwrap().unwrap();

    assert_eq!(shanten, 2);
    assert_eq!(discards, 0b0010101_000000000_101011010_000000000);
    assert_eq!(waits, 0b0000101_000000000_111111111_000000000);
}
