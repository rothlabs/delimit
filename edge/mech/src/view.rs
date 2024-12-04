use super::*;
mod make;

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
    pub fn plot(&self, plot: impl Into<Hub<Plot>>) -> make::Chart {
        make::Chart {
            view: self.clone(),
            plot: plot.into(),
        }
    }
}
