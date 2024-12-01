pub use bank::ComputeProgram;

use super::*;
use bank::*;

mod bank;
mod make;

#[derive(Clone, Debug)]
pub struct Core {
    pub gpu: Gpu,
    pub store: gpu::Store,
    pub bank: Grc<Bank>,
}

impl Core {
    pub fn new(port: &Viewport) -> Result<Self> {
        let store = gpu::store(&port.gpu);
        Ok(Self {
            bank: Bank::new(&store, port)?.into(),
            gpu: port.gpu.clone(),
            store,
        })
    }
    pub fn travel(&self) -> TravelBuilder {
        TravelBuilder::default()
    }
    pub fn orient(&self) -> OrientBuilder {
        OrientBuilder::default()
    }
    pub fn spline(&self) -> SplineBuilder {
        SplineBuilder::default()
    }
    pub fn form(&self) -> FormBuilder {
        FormBuilder::default()
    }
    pub fn flow(&self) -> FlowBuilder {
        FlowBuilder::default()
    }
    /// Shape builder of given dimensionality.
    pub fn shape(&self, dimension: u32) -> ShapeBuilder {
        ShapeBuilder::default().dimension(dimension)
    }
    pub fn plot(&self, shape: impl Into<Hub<Shape>>) -> make::Chart {
        make::Chart {
            core: self.clone(),
            shape: shape.into(),
        }
    }
}

// // TODO: also impl on shape to creating everything needed to render automatically
// pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> make::Draw {
//     make::Draw {
//         core: self.clone(),
//         plot: plot.into(),
//     }
// }
