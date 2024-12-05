use super::*;

mod chart;

pub struct ChartView {
    pub view: View,
    pub chart: Hub<Chart>,
}

impl ChartView {
    pub fn points(self) -> chart::PointsBuilder {
        chart::PointsBuilder::default()
            .view(self.view)
            .plot(self.chart)
    }
}
