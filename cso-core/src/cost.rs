#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Cost(f64);

impl Cost {
    pub const fn new(val: f64) -> Cost {
        Cost(val)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
