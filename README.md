# Shanten DP Rust

[![Crate](https://img.shields.io/crates/v/shanten-dp.svg)](https://crates.io/crates/shanten-dp)
[![Minimum Supported Rust Version](https://img.shields.io/crates/msrv/shanten-dp)](https://crates.io/crates/shanten-dp)
[![API](https://docs.rs/shanten-dp/badge.svg)](https://docs.rs/shanten-dp)

Rust implementation of the Shanten calculation algorithm using dynamic programming.

## Overview

This project provides an efficient algorithm to calculate the minimum number of tiles needed to form a tenpai hand in Mahjong, known as "Shanten" (向聴).

## Features

- Provably correct (理論的な正確性を保証)
- Fast DP-based computation (動的計画法ベースの高速計算)
- Supports arbitrary hand sizes (少牌/多牌に対応)
- Supports tile availability constraints (残り枚数の制約に対応)
  - Supports open melds (副露に対応)
  - Supports three-player mahjong (三人麻雀に対応)

## Usage

### Examples

```rust
use shanten_dp::{calc_shanten, make_tile_limits};

fn main() {
    // 123m245779p13555z
    let hand: [i8; 34] = [
        1, 1, 1, 0, 0, 0, 0, 0, 0, // manzu
        0, 1, 0, 1, 1, 0, 2, 0, 1, // pinzu
        0, 0, 0, 0, 0, 0, 0, 0, 0, // souzu
        1, 0, 1, 0, 3, 0, 0, // jihai
    ];
    let tile_limits = make_tile_limits(false);
    let shanten = calc_shanten(&hand, &tile_limits, 4, 7, true).unwrap();

    assert!(matches!(shanten, Some(2)));
}
```

### Benchmarks

1. Put Ara's test cases (`p_normal_10000.txt`, `p_hon_10000.txt`, `p_tin_10000.txt`, and `p_koku_10000.txt`) in [benches/data](benches/data)[^1].
1. Run `cargo bench`.

[^1]: Available from <https://mahjong.ara.black/etc/shanten/shanten9.htm>.
