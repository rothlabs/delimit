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
    pub fn extrude(mut self, hedge: Hedge) -> Self {
        self = self.setup_form();
        if let Some(form) = &mut self.form {
            form.linear_setup();
            if let Some(add) = &mut form.linear {
                add.extrude = Some(hedge);
            }
        }
        self
    }
    pub fn basis(mut self, hedge: Hedge, order: usize) -> Self {
        self = self.setup_form();
        if let Some(form) = &mut self.form {
            form.spline_setup(order);
            if let Some(Some(right)) = form.spline.get_mut(order) {
                right.basis = Some(hedge);
            }
        }
        self
    }
    pub fn nurbs(mut self, hedge: Hedge, order: usize) -> Self {
        self = self.setup_form();
        if let Some(form) = &mut self.form {
            form.spline_setup(order);
            if let Some(Some(right)) = form.spline.get_mut(order) {
                right.nurbs = Some(hedge);
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
    linear: Option<form::Linear>,
    // matched to vector control
    // matrix: form::Matrix,
    // matched to matrix control indexed by order
    spline: Vec<Option<form::Spline>>,
}

impl Form {
    fn linear_setup(&mut self) {
        if self.linear.is_none() {
            self.linear = Some(form::Linear::default());
        }
    }
    fn spline_setup(&mut self, order: usize) {
        if order > self.spline.len() {
            let fill = order - self.spline.len();
            for _ in 0..fill {
                self.spline.push(None);
            }
        }
        if self.spline.len() == order {
            self.spline.push(Some(form::Spline::default()));
        }
    }
}

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Flow {
    #[builder(default)]
    linear: Option<Hedge>,
    // index hedge matched to matrix span
    // vector: Option<Hedge>,
    // index hedge matched to vector span indexed by order
    #[builder(default)]
    _spline: Vec<Option<Hedge>>,
    // pick translation
    // pick matrix
    // pick translation and matrix
    //
}

impl FlowBuilder {
    // pub fn extrude(self, hedge: Hedge) -> Self {
    //     self.add(hedge)
    // }
    pub fn spline(mut self, hedge: Hedge, order: usize) -> Self {
        if self._spline.is_none() {
            self = self._spline(vec![]);
        }
        if let Some(matrix) = &mut self._spline {
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
