use super::*;

mod chart;

pub struct Chart {
    pub core: Core,
    pub shape: Hub<Shape>,
}

impl Chart {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> chart::GridBuilder {
        chart::GridBuilder::default()
            .core(self.core)
            .shape(self.shape)
            .count(count)
    }
}
