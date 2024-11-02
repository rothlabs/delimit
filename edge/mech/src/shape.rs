use super::*;

mod arch;
mod make;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    warp: Hedge,
    arch: Arch,
    // alt: Pile or Stud
    jambs: Vec<Jamb>,
    #[builder(default = "2")]
    pub dimension: u32,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

impl Shape {
    pub fn plot<'a>(&'a self, core: &'a Core) -> make::Plot<'a> {
        make::Plot { core, shape: self }
    }
    pub fn plot_stride(&self) -> u32 {
        self.dimension + self.dimension * self.rank()
    }
    pub fn rank(&self) -> u32 {
        self.jambs.len() as u32
    }
}

#[derive(Clone, Debug)]
struct Arch {
    // matched to vector control
    // matrix: arch::Matrix,
    // matched to matrix control indexed by order
    vector: Vec<Option<arch::Vector>>,
}

#[derive(Clone, Debug)]
struct Jamb {
    // index hedge matched to matrix span
    // vector: Option<Hedge>,
    // index hedge matched to vector span indexed by order
    matrix: Vec<Option<Hedge>>,
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
