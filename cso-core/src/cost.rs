use std::ops::{Add, AddAssign};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct Cost(f64);

impl Cost {
    pub const fn new(val: f64) -> Cost {
        Cost(val)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl AddAssign for Cost {
    fn add_assign(&mut self, rhs: Cost) {
        self.0 = self.0 + rhs.0
    }
}

impl Add for Cost {
    type Output = Cost;

    fn add(self, rhs: Cost) -> Self::Output {
        Cost::new(self.0 + rhs.0)
    }
}
