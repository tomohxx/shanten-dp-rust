pub const NUM_TIDS: usize = 34;
pub const MAX_SHT: i8 = 14;

#[inline]
pub fn chmin(x: &mut i8, y: i8) {
    if *x > y {
        *x = y;
    }
}

#[inline]
pub fn get_next_value(current: i8, distance: i8) -> i8 {
    current + distance.max(0)
}
