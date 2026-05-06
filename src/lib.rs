mod common;

pub mod seven_pairs;
pub mod standard;
pub mod thirteen_orphans;

mod shanten;
pub use shanten::ShantenError;
pub use shanten::calc_shanten;
pub use shanten::make_tile_limits;
