#![no_std]
#![doc = include_str!("../README.md")]

mod common;

mod seven_pairs;
mod standard;
mod thirteen_orphans;

mod shanten;
pub use shanten::ShantenError;
pub use shanten::calc_shanten;
pub use shanten::make_tile_limits;
