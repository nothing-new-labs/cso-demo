#[derive(PartialEq, PartialOrd)]
pub struct Cost {
    cost_value: f64,
}

impl Cost {
    pub const fn new() -> Cost {
        Cost { cost_value: 0.0 }
    }

    pub fn cost_value(&self) -> f64 {
        self.cost_value
    }
}
