#![doc = include_str!("../README.md")]

mod common;

#[doc(hidden)]
pub mod seven_pairs;
#[doc(hidden)]
pub mod standard;
#[doc(hidden)]
pub mod thirteen_orphans;

mod shanten;
pub use shanten::ShantenError;
pub use shanten::calc_shanten;
pub use shanten::make_tile_limits;
