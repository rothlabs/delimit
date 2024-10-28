use super::*;
// use plot::Plot;

mod plot;

#[derive(Clone, Debug)]
pub struct Core {
    pub gpu: Gpu,
    pub bin: Grc<Bin>,
}

impl Core {
    pub fn new(gpu: Gpu) -> graph::Result<Self> {
        Ok(Self {
            bin: Bin::new(&gpu)?.into(),
            gpu,
        })
    }
    pub fn shape(&self, rule: Rule) -> ShapeBuilder {
        ShapeBuilder::default().rule(rule)
    }
    pub fn plot(&self, shape: impl Into<Hub<Shape>>) -> plot::Plot {
        plot::Plot {
            core: self.clone(),
            shape: shape.into(),
        }
    }
    pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> PointsBuilder {
        PointsBuilder::default().core(self.clone()).plot(plot)
    }
}
