use super::*;

mod chart;

pub struct Image<'a> {
    pub view: &'a View,
    pub chart: Hub<Chart>,
}

impl Image<'_> {
    pub fn points(&self) -> Hub<Grc<Command>> {
        chart::Points {
            view: self.view.clone(),
            chart: self.chart.clone(),
        }
        .hub()
    }
}

// gpu::Action

// chart::PointsBuilder::default()
//             .view(self.view)
//             .chart(self.chart)
