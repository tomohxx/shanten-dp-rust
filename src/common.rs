pub const NUM_TIDS: usize = 34;
pub const MAX_SHT: u8 = 100;

#[inline]
pub fn chmin(x: &mut u8, y: u8) {
    if *x > y {
        *x = y;
    }
}
