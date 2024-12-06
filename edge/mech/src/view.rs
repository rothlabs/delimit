use super::*;

mod hub;

#[derive(Clone, Debug)]
pub struct View {
    pub mech: Mech,
    pub port: Viewport,
}

impl View {
    pub fn new(mech: Mech, port: Viewport) -> Self {
        Self { mech, port }
    }
    // TODO: also impl on shape to create everything needed to render automatically
    pub fn image(&self, chart: impl Into<Hub<Chart>>) -> hub::Image {
        hub::Image {
            view: self,
            chart: chart.into(),
        }
    }
}
