pub use grid::*;

use super::*;

mod grid;

pub struct Plot {
    pub mech: Mech,
    pub shape: Hub<Shape>,
}

impl Plot {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> graph::Result<Hub<Hedge>> {
        GridBuilder::default()
            .mech(self.mech)
            .shape(self.shape)
            .count(count.into())
            .hub()
    }
}
