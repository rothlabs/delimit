use super::*;

mod make;
mod span;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    span: Span,
    points: Hedge,
    control: Vec<Control>,
    #[builder(default = "2")]
    pub dimension: u32,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

impl Shape {
    pub fn plot<'a>(&'a self, mech: &'a Core) -> make::Plot<'a> {
        make::Plot {
            bank: &mech.bank,
            gpu: &mech.gpu,
            shape: self,
        }
    }
    pub fn plot_stride(&self) -> u32 {
        self.dimension + self.dimension * self.rank()
    }
    pub fn rank(&self) -> u32 {
        self.control.len() as u32
    }
}

#[derive(Clone, Debug)]
struct Span {
    // matched to vector control
    matrix: Vec<span::Matrix>,
    vector: Vec<Vec<span::Vector>>,
}

#[derive(Clone, Debug)]
struct Control {
    vector: Hedge,
    matrix: Vec<Hedge>,
}

// #[derive(Clone, Debug)]
// pub struct Instance {
//     pub hedge: Hedge,
//     pub layout: Layout,
//     pub instance: Option<Box<Instance>>,
// }

#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}

// #[derive(Clone, Debug)]
// pub enum Rule {
//     Nurbs(u32),
//     Extrude,
//     Revolve,
// }

// #[derive(Clone, Debug)]
// pub enum ControlOld {
//     Shape(Vec<Shape>),
//     Hedge(Hedge),
// }

// impl ControlOld {
//     fn rank(&self, rank: u32) -> u32 {
//         match self {
//             Self::Hedge(_) => rank,
//             Self::Shape(shape) => {
//                 if let Some(shape) = shape.first() {
//                     shape.control.rank(rank) + rank
//                 } else {
//                     rank
//                 }
//             }
//         }
//     }
// }