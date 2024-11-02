use super::*;

mod arch;
mod make;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    warp: Hedge,
    #[builder(default)]
    arch: Arch,
    // alt: Pile or Stud
    #[builder(default)]
    jambs: Vec<Jamb>,
    dimension: u32,
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

impl ShapeBuilder {
    pub fn nurbs(mut self, order: usize, hedge: Hedge) -> Self {
        if let Some(arch) = &mut self.arch {
            arch.setup_vector(order);
            if let Some(Some(vector)) = arch.vector.get_mut(order) {
                vector.nurbs = Some(hedge);
            }
        }
        self
    }
    pub fn matrix(mut self, order: usize, hedge: Hedge) -> Self {
        
        self
    }
}

#[derive(Clone, Default, Debug)]
struct Arch {
    // matched to vector control
    // matrix: arch::Matrix,
    // matched to matrix control indexed by order
    vector: Vec<Option<arch::Vector>>,
}

impl Arch {
    fn setup_vector(&mut self, order: usize) {
        for _ in 0..order - self.vector.len() {
            self.vector.push(None);
        }
        if self.vector.len() == order { 
            self.vector.push(Some(arch::Vector::default()));
        }
    }
}

#[derive(Clone, Default, Debug)]
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
