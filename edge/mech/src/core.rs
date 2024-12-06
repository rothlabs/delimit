use super::*;

mod pipe;
mod hub;

#[derive(Clone, Debug)]
pub struct Mech {
    pub gpu: Gpu,
    pub bank: Grc<Pipe>,
}

impl Mech {
    pub fn new(port: &Viewport) -> Result<Self> {
        Ok(Self {
            bank: Pipe::new(port)?.into(),
            gpu: port.gpu.clone(),
        })
    }
    pub fn shape(&self, dimension: u32) -> ShapeBuilder {
        ShapeBuilder::default().dimension(dimension)
    }
    pub fn chart(&self, shape: impl Into<Hub<Shape>>) -> hub::Chart {
        hub::Chart {
            mech: self,
            shape: shape.into(),
        }
    }
}

#[derive(Debug)]
pub struct Pipe {
    pub chart: pipe::Chart,
    pub image: pipe::Image,
}

impl Pipe {
    pub fn new(port: &Viewport) -> Result<Self> {
        Ok(Self {
            chart: pipe::Chart::new(&port.gpu)?,
            image: pipe::Image::new(port)?,
        })
    }
}

// pub fn travel(&self) -> TravelBuilder {
    //     TravelBuilder::default()
    // }
    // pub fn orient(&self) -> OrientBuilder {
    //     OrientBuilder::default()
    // }
    // pub fn spline(&self) -> SplineBuilder {
    //     SplineBuilder::default()
    // }
    // pub fn form(&self) -> FormBuilder {
    //     FormBuilder::default()
    // }
    // pub fn flow(&self) -> FlowBuilder {
    //     FlowBuilder::default()
    // }
    // Shape builder with dimensionality.

// // TODO: also impl on shape to creating everything needed to render automatically
// pub fn draw(&self, plot: impl Into<Hub<Plot>>) -> make::Draw {
//     make::Draw {
//         core: self.clone(),
//         plot: plot.into(),
//     }
// }
