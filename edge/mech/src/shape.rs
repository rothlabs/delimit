// pub use form::*;

use super::*;

pub mod form;
mod hedge;

/// Continuous parametric geometry.
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
pub struct Shape {
    pub dimension: u32,
    pub warp: Hedge,
    pub form: Form,
    #[builder(setter(each(name = "flow", into)))]
    pub flows: Vec<Flow>,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
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

// impl ShapeBuilder {
//     pub fn travel(self, travel: Travel) -> Self {
//         self.mut_form(|form| form.travel = Some(travel))
//     }
//     pub fn orient(self, orient: Orient) -> Self {
//         self.mut_form(|form| form.orient = Some(orient))
//     }
//     pub fn spline(self, spline: Spline, order: usize) -> Self {
//         self.mut_form(|form| {
//             form.mut_spline(order, |form_spline| {
//                 *form_spline = spline;
//             });
//         })
//     }
//     pub fn extrude(self, hedge: Hedge) -> Self {
//         self.mut_form(|form| {
//             let mut travel = form.travel.take().unwrap_or_default();
//             travel.extrude = Some(hedge);
//             form.travel = Some(travel);
//         })
//     }
//     pub fn revolve(self, hedge: Hedge) -> Self {
//         self.mut_form(|form| {
//             let mut orient = form.orient.take().unwrap_or_default();
//             orient.revolve = Some(hedge);
//             form.orient = Some(orient);
//         })
//     }
//     // pub fn basis(self, hedge: Hedge, order: usize) -> Self {
//     //     self.mut_form(|form| {
//     //         form.mut_spline(order, |spline| {
//     //             spline.basis = Some(hedge);
//     //         });
//     //     })
//     // }
//     pub fn nurbs(self, hedge: Hedge, order: usize) -> Self {
//         self.mut_form(|form| {
//             form.splines.push(value);
//             // form.mut_spline(order, |spline| {
//             //     spline.nurbs = Some(hedge);
//             // });
//         })
//     }
//     fn mut_form<F: FnOnce(&mut Form)>(mut self, func: F) -> Self {
//         let mut form = self.form.take().unwrap_or_default();
//         func(&mut form);
//         self.form = Some(form);
//         self
//     }
// }

/// Specify how to create `Weft` evaluations.
#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Form {
    pub travel: Option<form::Travel>,
    pub orient: Option<form::Orient>,
    #[builder(setter(each(name = "spline")))]
    pub splines: Vec<form::Spline>,
}

// impl Form {
//     fn mut_spline<F: FnOnce(&mut Spline)>(&mut self, order: usize, func: F) {
//         while self.splines.len() < order + 1 {
//             self.splines.push(None);
//         }
//         let mut spline = self.splines[order].take().unwrap_or_default();
//         func(&mut spline);
//         self.splines[order] = Some(spline);
//     }
// }

/// Index into Warp and `Weft`.
#[derive(Builder, Clone, Debug, Default)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Flow {
    #[builder(default)]
    pub travel: Option<Hedge>,
    #[builder(default)]
    pub orient: Option<Hedge>,
    #[builder(default)]
    pub splines: Vec<flow::Spline>,
}

// impl FlowBuilder {
//     pub fn spline(mut self, hedge: Hedge, order: usize) -> Self {
//         let mut splines = self.splines.take().unwrap_or_default();
//         while splines.len() < order + 1 {
//             splines.push(None);
//         }
//         splines[order] = Some(hedge);
//         self.splines(splines)
//     }
// }

// instance layout
// #[derive(Clone, Debug)]
// pub enum Layout {
//     Free,
//     Grid,
//     Radial,
// }

pub mod flow {
    use super::*;
    #[derive(Debug, Clone)]
    pub struct Spline {
        pub order: u32,
        pub hedge: Hedge,
    }
}

// #[derive(Clone, Debug)]
// pub struct Instance {
//     pub hedge: Hedge,
//     pub layout: Layout,
//     pub instance: Option<Box<Instance>>,
// }
