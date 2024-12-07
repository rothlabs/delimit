// use layout::GroupLayout;
use super::*;

mod hub;
pub(crate) mod pipe;
// mod layout;
mod bind;
mod group;

#[derive(Clone, Debug)]
pub struct Mech {
    pub gpu: Gpu,
    pub pipe: Grc<Pipe>,
}

// TODO: Mech should be made from Gpu and mech::View should be made from gpu::Viewport
// mech can still contain all the Pipe info
impl Mech {
    pub fn new(gpu: &Gpu) -> Self {
        let layout = group::layout::Bank::new(gpu);
        Self {
            gpu: gpu.clone(),
            pipe: Pipe::new(&layout).into(),
        }
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
struct Pipe {
    pub chart: pipe::Chart,
    pub image: pipe::Image,
}

impl Pipe {
    pub fn new(layout: &group::layout::Bank) -> Self {
        Self {
            chart: pipe::Chart::new(layout),
            image: pipe::Image::new(layout),
        }
    }
}

#[derive(Debug)]
struct Bind {
    // pub topic: bind::Topic,
    // pub image: pipe::Image,
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
