use super::*;

mod chart;
mod draw;

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

pub struct Draw {
    pub core: Core,
    pub plot: Hub<Plot>,
}

impl Draw {
    pub fn points(self) -> draw::PointsBuilder {
        draw::PointsBuilder::default()
            .core(self.core)
            .plot(self.plot)
    }
}
