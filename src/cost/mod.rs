#[repr(transparent)]
pub struct Cost(f64);

impl Cost {
    pub const fn new() -> Cost {
        Cost(0.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}
