pub use vector3::*;
pub use triangle::*;

use std::ops::*;

mod vector3;
mod triangle;

pub trait Number: Copy + Add<Self, Output = Self> {}