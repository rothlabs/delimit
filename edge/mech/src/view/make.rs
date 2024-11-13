use super::*;

mod chart;

pub struct Chart {
    pub view: View,
    pub plot: Hub<Plot>,
}

impl Chart {
    pub fn points(self) -> chart::PointsBuilder {
        chart::PointsBuilder::default()
            .view(self.view)
            .plot(self.plot)
    }
}
