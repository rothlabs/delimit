use super::*;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    gpu: Gpu,
    #[builder(default = "2")]
    dimension: u32,
    rule: Rule,
    plan: Hedge,
    control: Control,
    index: Hedge,
    // #[builder(default)]
    // bounds: Vec<Shape>,
    // #[builder(default)]
    // instance: Option<Instance>,
}

// impl ShapeBuilder {
//     pub fn nurbs(self, order: u64) -> Self {
//         self.rule(Rule::Nurbs(order))
//     }
// }

impl Shape {
    pub fn grid(&self, count: Hub<u32>) -> graph::Result<Hedge> {
        // let table = self.table.base().await?;
        // if let Table::Hedge(hedge) = table {
        //     let buffer = hedge.buffer.base().await?;
        //     let size = buffer.size() * count / 3;
        //     let basis = self.gpu.buffer(size).storage_copy()?;
        if let Rule::Nurbs(order) = self.rule {
            return self.nurbs_grid(order, count);
        }
        // }
        Err(anyhow!("grid plot not implemented for this shape"))?
    }
    fn nurbs_grid(&self, order: u32, count: Hub<u32>) -> graph::Result<Hedge> {
        let setup_entry = self.gpu.uniform().entry(0)?.compute()?;
        let plan_entry = self.gpu.storage(true).entry(1)?.compute()?;
        let basis_entry = self.gpu.storage(false).entry(2)?.compute()?;
        let bind_layout = self
            .gpu
            .bind_layout(&[setup_entry, plan_entry, basis_entry])
            .make()?;
        let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
        let shader = self.gpu.shader(include_wgsl!("plot/nurbs_grid.wgsl"));
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let setup = self
            .gpu
            .buffer_uniform()
            .field(order)
            .field(count.clone())
            .hub()?;
        let basis = self
            .gpu
            .sizer(self.plan.buffer.clone())
            .mul(count.clone())
            .mul(2)
            .div(3)
            .hub()?;
        let bind = self
            .gpu
            .binder()
            .layout(bind_layout)
            .entry(0, setup)
            .entry(1, self.plan.buffer.clone())
            .entry(2, basis.clone())
            .hub()?;
        let compute = self
            .gpu
            .compute()
            .root(self.plan.mutator.clone())
            .pipe(pipe)
            .bind(bind)
            .dispatch(count.clone());
        match &self.control {
            Control::Shape(shapes) => {
                let shape = shapes.first().unwrap();
                let _ = shape.grid(count)?;
                panic!("not implemented");
            }
            Control::Hedge(control) => {
                let setup_entry = self.gpu.uniform().entry(0)?.compute()?;
                let index_entry = self.gpu.storage(true).entry(1)?.compute()?;
                let basis_entry = self.gpu.storage(true).entry(2)?.compute()?;
                let control_entry = self.gpu.storage(true).entry(3)?.compute()?;
                let plot_entry = self.gpu.storage(false).entry(4)?.compute()?;
                let bind_layout = self
                    .gpu
                    .bind_layout(&[
                        setup_entry,
                        index_entry,
                        basis_entry,
                        control_entry,
                        plot_entry,
                    ])
                    .make()?;
                let stride = 1;
                let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
                let shader = self.gpu.shader(include_wgsl!("plot/control_matrix.wgsl"));
                let pipe = shader.compute("main").layout(&pipe_layout).make()?;
                let setup = self
                    .gpu
                    .buffer_uniform()
                    .field(order)
                    .field(count.clone())
                    .field(stride)
                    .field(self.dimension)
                    .hub()?;
                let plot = self
                    .gpu
                    .sizer(basis.clone())
                    .mul(self.dimension)
                    .mul(stride)
                    .div(order)
                    .hub()?;
                let bind = self
                    .gpu
                    .binder()
                    .layout(bind_layout)
                    .entry(0, setup)
                    .entry(1, self.index.buffer.clone())
                    .entry(2, basis)
                    .entry(3, control.buffer.clone())
                    .entry(4, plot.clone())
                    .hub()?;
                let mutator = compute
                    .root(self.index.mutator.clone())
                    .root(control.mutator.clone())
                    .pipe(pipe)
                    .bind(bind)
                    .dispatch(count.clone())
                    .hub()?;
                return Ok(Hedge {
                    buffer: plot,
                    mutator,
                });
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

#[derive(Clone, Debug)]
pub struct Instance {
    pub table: Table,
    pub layout: Layout,
    pub instance: Option<Box<Instance>>,
}

#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}
