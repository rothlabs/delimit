use super::*;

mod chart;
mod draw;

pub struct Chart {
    pub core: Core,
    pub shape: Hub<Shape>,
}

impl Chart {
    pub fn grid(self, count: impl Into<Hub<u32>>) -> graph::Result<Hub<Plot>> {
        chart::GridBuilder::default()
            .core(self.core)
            .shape(self.shape)
            .count(count)
            .hub()
    }
}

pub struct Draw {
    pub core: Core,
    pub plot: Hub<Plot>,
}

impl Draw {
    pub fn points(self) -> draw::PointsBuilder {
        // , size: impl Into<Hub<f32>>
        draw::PointsBuilder::default()
            .core(self.core)
            .plot(self.plot)
        // .size(size)
    }
}

// Ok(Grid {
//     mech: self.mech,
//     shape: self.shape,
//     count: count.into(),
// }.gate()?.into())
