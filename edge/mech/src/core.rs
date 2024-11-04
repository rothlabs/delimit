use super::*;

mod make;

#[derive(Clone, Debug)]
pub struct Core {
    pub gpu: Gpu,
    pub bank: Grc<Bank>,
}

impl Core {
    pub fn new(gpu: Gpu) -> graph::Result<Self> {
        Ok(Self {
            bank: Bank::new(&gpu)?.into(),
            gpu,
        })
    }
    pub fn flow(&self) -> FlowBuilder {
        FlowBuilder::default()
    }
    pub fn shape(&self, dimension: u32) -> ShapeBuilder {
        ShapeBuilder::default().dimension(dimension)
    }
    pub fn chart(&self, shape: impl Into<Hub<Shape>>) -> make::Chart {
        make::Chart {
            core: self.clone(),
            shape: shape.into(),
        }
    }
    // TODO: also impl on shape to creating everything needed to render automatically
    pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> make::Draw {
        make::Draw {
            core: self.clone(),
            plot: plot.into(),
        }
    }
}

// pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> plot::draw::PointsBuilder {
//     plot::draw::PointsBuilder::default().core(self.clone()).plot(plot)
// }
