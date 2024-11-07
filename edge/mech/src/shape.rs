pub use form::*;

use super::*;

mod form;
mod make;

const EXISTS: &str = "Field must exist by ensuring default.";

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
        self.dimension + self.dimension * self.rank()
    }
    pub fn rank(&self) -> u32 {
        self.flows.len() as u32
    }
}

impl ShapeBuilder {
    pub fn travel(self, travel: form::Travel) -> Self {
        self.mut_form(|form| {
            form.travel = Some(travel);
        })
    }
    pub fn extrude(self, hedge: Hedge) -> Self {
        self.mut_form(|form| {
            form.mut_travel(|travel| {
                travel.extrude = Some(hedge);
            });
        })
    }
    pub fn revolve(self, hedge: Hedge) -> Self {
        self.mut_form(|form| {
            form.mut_orient(|orient| {
                orient.revolve = Some(hedge);
            });
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
        if self.form.is_none() {
            self.form = Some(Form::default());
        }
        func(self.form.as_mut().expect(EXISTS));
        self
    }
}

#[derive(Clone, Default, Debug)]
struct Form {
    travel: Option<form::Travel>,
    orient: Option<form::Orient>,
    spline: Vec<Option<form::Spline>>,
}

impl Form {
    fn mut_travel<F: FnOnce(&mut form::Travel)>(&mut self, func: F) {
        if self.travel.is_none() {
            self.travel = Some(form::Travel::default());
        }
        func(self.travel.as_mut().expect(EXISTS));
    }
    fn mut_orient<F: FnOnce(&mut form::Orient)>(&mut self, func: F) {
        if self.orient.is_none() {
            self.orient = Some(form::Orient::default());
        }
        func(self.orient.as_mut().expect(EXISTS));
    }
    fn mut_spline<F: FnOnce(&mut form::Spline)>(&mut self, order: usize, func: F) {
        while self.spline.len() < order + 1 {
            self.spline.push(None);
        }
        let mut spline = self.spline[order].take();
        if spline.is_none() {
            spline = Some(form::Spline::default());
        }
        func(spline.as_mut().expect(EXISTS));
        self.spline[order] = spline;
    }
}

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
    _spline: Vec<Option<Hedge>>,
}

impl FlowBuilder {
    pub fn spline(mut self, hedge: Hedge, order: usize) -> Self {
        if self._spline.is_none() {
            self = self._spline(vec![]);
        }
        // let mut spline = self._spline.take().expect(EXISTS);
        // while spline.len() < order + 1 {
        //     spline.push(None);
        // }
        // spline[order] = Some(hedge);
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
