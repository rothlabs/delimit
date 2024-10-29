use super::*;

mod make;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    // span: Span {
    // vector: Vec<Vector> { (basis) different vector slots for different vector length (order)
    // rule: VectorRule (direct, nurbs)
    // hedge: Hedge
    // }
    // matrix
    // }
    rule: Rule,
    span: Hedge,
    index: Hedge,
    control: Control,
    // points: Hedge (base controls)
    // control: Vec<Control> { (list by rank)
    // vector: Hedge (index like [matrix_span_index, control_index])
    // matrix: Hedge (index like [vector_span_index, control_index]) (different matrix slots for different matrix width (order))
    // }
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
        self.control.rank(1)
    }
}

#[derive(Clone, Debug)]
pub enum Rule {
    Nurbs(u32),
    Extrude,
    Revolve,
}

#[derive(Clone, Debug)]
pub enum Control {
    Shape(Vec<Shape>),
    Hedge(Hedge),
}

impl Control {
    fn rank(&self, rank: u32) -> u32 {
        match self {
            Self::Hedge(_) => rank,
            Self::Shape(shape) => {
                if let Some(shape) = shape.first() {
                    shape.control.rank(rank) + rank
                } else {
                    rank
                }
            }
        }
    }
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
