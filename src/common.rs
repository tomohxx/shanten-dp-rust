pub(crate) const NUM_TIDS: usize = 34;
pub(crate) const MAX_SHT: i8 = 100;

pub trait Calculatable: Copy + PartialEq<i8> {
    fn new(shanten: i8) -> Self;
    fn chmin(&mut self, other: Self);
    fn get_next_value(&self, distance: i8, n: usize) -> Self;
}

impl Calculatable for i8 {
    #[inline]
    fn new(shanten: i8) -> Self {
        shanten
    }

    #[inline]
    fn chmin(&mut self, other: Self) {
        if *self > other {
            *self = other;
        }
    }

    #[inline]
    fn get_next_value(&self, distance: i8, _n: usize) -> Self {
        self + distance.max(0) as i8
    }
}

/// Calculation result for [`crate::calc_shanten2`].
#[derive(Clone, Copy, Debug)]
pub struct Data {
    pub shanten: i8,
    pub discards: u64,
    pub waits: u64,
}

impl PartialEq<i8> for Data {
    fn eq(&self, other: &i8) -> bool {
        self.shanten == *other
    }
}

impl Calculatable for Data {
    fn new(shanten: i8) -> Self {
        Self { shanten, discards: 0, waits: 0 }
    }

    fn chmin(&mut self, other: Self) {
        if self.shanten > other.shanten {
            *self = other;
        } else if self.shanten == other.shanten {
            self.discards |= other.discards;
            self.waits |= other.waits;
        }
    }

    fn get_next_value(&self, distance: i8, n: usize) -> Self {
        Self {
            shanten: self.shanten + distance.max(0) as i8,
            discards: if distance < 0 { self.discards | 1u64 << n } else { self.discards },
            waits: if distance > 0 { self.waits | 1u64 << n } else { self.waits },
        }
    }
}
