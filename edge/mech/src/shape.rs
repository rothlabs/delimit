use super::*;

mod form;
mod make;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct Shape {
    warp: Hedge,
    form: Form,
    #[builder(setter(each(name = "flow", into)))]
    flows: Vec<Flow>,
    dimension: u32,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

impl Shape {
    pub fn chart<'a>(&'a self, core: &'a Core) -> make::Chart<'a> {
        make::Chart { core, shape: self }
    }
    pub fn plot_stride(&self) -> u32 {
        self.dimension + self.dimension * self.flows.len() as u32
    }
    // pub fn rank(&self) -> u32 {
    //     self.flows.len() as u32
    // }
}

impl ShapeBuilder {
    pub fn spline(mut self, order: usize, hedge: Hedge) -> Self {
        self = self.setup_form();
        if let Some(form) = &mut self.form {
            form.setup_vector(order);
            if let Some(Some(vector)) = form.vector.get_mut(order) {
                vector.spline = Some(hedge);
            }
        }
        self
    }
    pub fn nurbs(mut self, order: usize, hedge: Hedge) -> Self {
        self = self.setup_form();
        if let Some(form) = &mut self.form {
            form.setup_vector(order);
            if let Some(Some(vector)) = form.vector.get_mut(order) {
                vector.nurbs = Some(hedge);
            }
        }
        self
    }
    fn setup_form(mut self) -> Self {
        if self.form.is_none() {
            self = self.form(Form::default())
        }
        self
    }
}

#[derive(Clone, Default, Debug)]
struct Form {
    // matched to vector control
    // matrix: form::Matrix,
    // matched to matrix control indexed by order
    vector: Vec<Option<form::Vector>>,
}

impl Form {
    fn setup_vector(&mut self, order: usize) {
        if order > self.vector.len() {
            let fill = order - self.vector.len();
            for _ in 0..fill {
                self.vector.push(None);
            }
        }
        if self.vector.len() == order {
            self.vector.push(Some(form::Vector::default()));
        }
    }
}

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
// #[builder(setter(into, strip_option))]
pub struct Flow {
    // index hedge matched to matrix span
    // vector: Option<Hedge>,
    // index hedge matched to vector span indexed by order
    // #[builder(default)]
    matrices: Vec<Option<Hedge>>,
}

impl FlowBuilder {
    pub fn matrix(mut self, order: usize, hedge: Hedge) -> Self {
        if self.matrices.is_none() {
            self = self.matrices(vec![]);
        }
        if let Some(matrix) = &mut self.matrices {
            for _ in 0..order - matrix.len() {
                matrix.push(None);
            }
            matrix.push(Some(hedge));
        }
        self
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
