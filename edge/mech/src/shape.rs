pub use control::*;

use super::*;

mod control;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    gpu: Gpu,
    mech: Mech,
    rule: Rule,
    span: Hedge,
    index: Hedge,
    control: Control,
    #[builder(default = "2")]
    dimension: u32,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

impl Shape {
    pub fn grid(&self, count: Hub<u32>) -> graph::Result<Hedge> {
        if let Rule::Nurbs(order) = self.rule {
            self.grid_nurbs(order, count)
        } else {
            Err(anyhow!("Only Rule::Nurbs implemented"))?
        }
    }
    fn grid_nurbs(&self, order: u32, count: Hub<u32>) -> graph::Result<Hedge> {
        let rig = self
            .gpu
            .uniform()
            .field(order)
            .field(count.clone())
            .make()?;
        let buffer = self
            .gpu
            .blank(self.span.buffer.clone())
            .mul(count.clone())
            .mul(2)
            .div(3)
            .hub()?;
        let bind = self
            .gpu
            .bind()
            .layout(self.mech.grid.basis.nurbs.layout.clone())
            .entry(0, rig.buffer)
            .entry(1, self.span.buffer.clone())
            .entry(2, buffer.clone())
            .hub()?;
        let root = self
            .gpu
            .command()
            .root(rig.root)
            .root(self.span.root.clone())
            .compute(self.mech.grid.basis.nurbs.pipe.clone())
            .bind(bind)
            .dispatch(count.clone())
            .hub()?;
        self.control.grid_basis(GridBasis {
            shape: self,
            order,
            count,
            basis: Hedge { buffer, root },
        })
    }
}

#[derive(Clone, Debug)]
pub enum Rule {
    Nurbs(u32),
    Extrude,
    Revolve,
}

// #[derive(Clone, Debug)]
// pub struct Instance {
//     pub table: Table,
//     pub layout: Layout,
//     pub instance: Option<Box<Instance>>,
// }

#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}
