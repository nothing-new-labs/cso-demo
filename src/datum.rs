use serde::{Deserialize, Serialize};

/// Datum is the struct to represent a single value in optimizer.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Datum {
    I32(i32),
}
