use super::*;

pub mod form;

mod hedge;

/// Continuous parametric geometry.
#[derive(Clone, Debug)]
pub struct Shape {
    pub dimension: u32,
    pub warp: Hedge,
    pub form: Form,
    pub flows: Vec<Flow>,
}

impl Shape {
    pub fn chart<'a>(&'a self, mech: &'a Mech) -> hedge::Chart<'a> {
        hedge::Chart { shape: self, mech }
    }
    pub fn plot_size(&self) -> u32 {
        self.dimension * (self.rank() + 1)
    }
    pub fn rank(&self) -> u32 {
        self.flows.len() as u32
    }
}

/// Specify how to create `Weft` evaluations.
#[derive(Clone, Debug, Default)]
pub struct Form {
    pub travel: Option<form::Travel>,
    pub orient: Option<form::Orient>,
    pub spline: BTreeMap<u32, form::Spline>,
}

// TODO: Flow should be in an enum of other kinds of operations like ButtJoin
/// Index into Warp and `Weft`.
#[derive(Clone, Debug, Default)]
pub struct Flow {
    pub travel: Option<Hedge>,
    pub orient: Option<Hedge>,
    pub spline: BTreeMap<u32, Hedge>,
}

// #[derive(Clone, Debug)]
// pub struct Instance {
//     pub hedge: Hedge,
//     pub layout: Layout,
//     pub instance: Option<Box<Instance>>,
// }

// instance layout
// #[derive(Clone, Debug)]
// pub enum Layout {
//     Free,
//     Grid,
//     Radial,
// }

// pub mod flow {
//     use super::*;
//     #[derive(Debug, Clone)]
//     pub struct Spline {
//         pub order: u32,
//         pub hedge: Hedge,
//     }
// }

// #[builder(default)]
// bounds: Vec<Shape>,
// #[builder(default)]
// instance: Option<Instance>,

// pub enum Form {
//     Travel(form::Travel),
//     Orient(form::Orient),
//     Spline(form::Spline),
// }

// #[derive(Clone, Debug)]
// pub enum Flow {
//     Travel(Hedge),
//     Orient(Hedge),
//     Spline(flow::Spline),
