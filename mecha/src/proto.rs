pub use vector3::*;
pub use triangle::*;
pub use plot::*;

use std::ops::*;

mod vector3;
mod triangle;
mod plot;

pub trait Number: Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>  {}

// impl Number for f32 {}
impl Number for f64 {}