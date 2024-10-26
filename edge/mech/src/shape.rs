use super::*;

mod plot;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    pub rule: Rule,
    pub span: Hedge,
    pub index: Hedge,
    pub control: Control,
    #[builder(default = "2")]
    pub dimension: u32,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

impl Shape {
    pub fn plot<'a>(&'a self, mech: &'a Mech) -> plot::Plot<'a> {
        plot::Plot {
            bin: &mech.bin,
            gpu: &mech.gpu,
            shape: self,
        }
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
