use super::*;

mod chart;

pub struct MechShape {
    pub core: Mech,
    pub shape: Hub<Shape>,
}

impl MechShape {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> chart::GridBuilder {
        chart::GridBuilder::default()
            .mech(self.core)
            .shape(self.shape)
            .count(count)
    }
}
