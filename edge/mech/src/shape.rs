pub use form::*;

use super::*;

mod form;
mod hedge;

/// Continuous parametric geometry.
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
    pub fn chart<'a>(&'a self, mech: &'a Mech) -> hedge::Chart<'a> {
        hedge::Chart { shape: self, mech}
    }
    pub fn plot_size(&self) -> u32 {
        self.dimension * (self.rank() + 1)
    }
    pub fn rank(&self) -> u32 {
        self.flows.len() as u32
    }
}

impl ShapeBuilder {
    pub fn travel(self, travel: Travel) -> Self {
        self.mut_form(|form| form.travel = Some(travel))
    }
    pub fn orient(self, orient: Orient) -> Self {
        self.mut_form(|form| form.orient = Some(orient))
    }
    pub fn spline(self, spline: Spline, order: usize) -> Self {
        self.mut_form(|form| {
            form.mut_spline(order, |form_spline| {
                *form_spline = spline;
            });
        })
    }
    pub fn extrude(self, hedge: Hedge) -> Self {
        self.mut_form(|form| {
            let mut travel = form.travel.take().unwrap_or_default();
            travel.extrude = Some(hedge);
            form.travel = Some(travel);
        })
    }
    pub fn revolve(self, hedge: Hedge) -> Self {
        self.mut_form(|form| {
            let mut orient = form.orient.take().unwrap_or_default();
            orient.revolve = Some(hedge);
            form.orient = Some(orient);
        })
    }
    pub fn basis(self, hedge: Hedge, order: usize) -> Self {
        self.mut_form(|form| {
            form.mut_spline(order, |spline| {
                spline.basis = Some(hedge);
            });
        })
    }
    pub fn nurbs(self, hedge: Hedge, order: usize) -> Self {
        self.mut_form(|form| {
            form.mut_spline(order, |spline| {
                spline.nurbs = Some(hedge);
            });
        })
    }
    fn mut_form<F: FnOnce(&mut Form)>(mut self, func: F) -> Self {
        let mut form = self.form.take().unwrap_or_default();
        func(&mut form);
        self.form = Some(form);
        self
    }
}

/// Specify how to create `Weft` evaluations.
#[derive(Builder, Clone, Default, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Form {
    travel: Option<form::Travel>,
    orient: Option<form::Orient>,
    #[builder(setter(each(name = "spline")))]
    splines: Vec<Option<form::Spline>>,
}

impl Form {
    fn mut_spline<F: FnOnce(&mut Spline)>(&mut self, order: usize, func: F) {
        while self.splines.len() < order + 1 {
            self.splines.push(None);
        }
        let mut spline = self.splines[order].take().unwrap_or_default();
        func(&mut spline);
        self.splines[order] = Some(spline);
    }
}

/// Index into Warp and `Weft`.
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Flow {
    #[builder(default)]
    travel: Option<Hedge>,
    #[builder(default)]
    orient: Option<Hedge>,
    #[builder(default)]
    splines: Vec<Option<Hedge>>,
}

impl FlowBuilder {
    pub fn spline(mut self, hedge: Hedge, order: usize) -> Self {
        let mut splines = self.splines.take().unwrap_or_default();
        while splines.len() < order + 1 {
            splines.push(None);
        }
        splines[order] = Some(hedge);
        self.splines(splines)
    }
}

// instance layout
#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}

// #[derive(Clone, Debug)]
// pub struct Instance {
//     pub hedge: Hedge,
//     pub layout: Layout,
//     pub instance: Option<Box<Instance>>,
// }
