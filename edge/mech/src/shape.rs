use super::*;

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
            self.nurbs_grid(order, count)
        } else {
            Err(anyhow!("grid plot not implemented for this shape"))?
        }
    }
    fn nurbs_grid(&self, order: u32, count: Hub<u32>) -> graph::Result<Hedge> {
        let rig = self.gpu.uniform().field(order).field(count.clone()).hub()?;
        let basis = self
            .gpu
            .blank(self.span.buffer.clone())
            .mul(count.clone())
            .mul(2)
            .div(3)
            .hub()?;
        let bind = self
            .gpu
            .binder()
            .layout(self.mech.grid.basis.nurbs.layout.clone())
            .entry(0, rig)
            .entry(1, self.span.buffer.clone())
            .entry(2, basis.clone())
            .hub()?;
        let command = self
            .gpu
            .command()
            .root(self.span.root.clone())
            .compute(self.mech.grid.basis.nurbs.pipe.clone())
            .bind(bind)
            .dispatch(count.clone());
        match &self.control {
            Control::Shape(shapes) => {
                let shape = shapes.first().unwrap();
                let _ = shape.grid(count)?;
                Err(anyhow!("Control::Shape not implemented"))?
            }
            Control::Hedge(control) => {
                let stride = 1;
                let rig = self
                    .gpu
                    .uniform()
                    .field(order)
                    .field(count.clone())
                    .field(stride)
                    .field(self.dimension)
                    .hub()?;
                let plot = self
                    .gpu
                    .blank(basis.clone())
                    .mul(self.dimension)
                    .mul(stride)
                    .div(order)
                    .hub()?;
                let bind = self
                    .gpu
                    .binder()
                    .layout(self.mech.grid.basis.control.layout.clone())
                    .entry(0, rig)
                    .entry(1, basis)
                    .entry(2, self.index.buffer.clone())
                    .entry(3, control.buffer.clone())
                    .entry(4, plot.clone())
                    .hub()?;
                let root = command
                    .root(self.index.root.clone())
                    .root(control.root.clone())
                    .compute(self.mech.grid.basis.control.pipe.clone())
                    .bind(bind)
                    .dispatch(count.clone())
                    .hub()?;
                Ok(Hedge { buffer: plot, root })
            }
        }
        // Err(anyhow!("grid plot not implemented for this shape"))?
    }
    // fn control_matrix(&self) -> graph::Result<Hedge> {

    // }
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

// impl ShapeBuilder {
//     pub fn nurbs(self, order: u64) -> Self {
//         self.rule(Rule::Nurbs(order))
//     }
// }
