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
    plan: Table,
    control: Control,
    #[builder(default)]
    bounds: Vec<Shape>,
    #[builder(default)]
    instance: Option<Instance>,
}

// impl ShapeBuilder {
//     pub fn nurbs(self, order: u64) -> Self {
//         self.rule(Rule::Nurbs(order))
//     }
// }

impl Shape {
    pub fn grid(&self, count: Hub<u32>) -> graph::Result<Hub<Hedge>> {
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
    fn nurbs_grid(&self, order: u32, count: Hub<u32>) -> graph::Result<Hub<Hedge>> {
        let setup = self.gpu.buffer_uniform().field(order).field(count.clone()).hub()?;
        if let Table::Hedge(plan) = &self.plan {
            let basis = self.gpu.sizer(plan.buffer.clone()).mul(count.clone()).mul(2).div(3).hub()?;
            let setup_entry = self.gpu.uniform().entry(0)?.compute()?;
            let plan_entry = self.gpu.storage(true).entry(1)?.compute()?;
            let basis_entry = self.gpu.storage(false).entry(2)?.compute()?;
            let bind_layout = self
                .gpu
                .bind_layout(&[setup_entry, plan_entry, basis_entry])
                .make()?;
            let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
            let nurbs_shader = self.gpu.shader(include_wgsl!("plot/nurbs_grid.wgsl"));
            let pipe = nurbs_shader.compute("main").layout(&pipe_layout).make()?;
            let bind = self
                .gpu
                .binder()
                .layout(bind_layout)
                .entry(0, setup)
                .entry(1, plan.buffer.clone())
                .entry(2, basis.clone())
                .hub()?;

            let mutator = self
                .gpu
                .compute()
                .root(plan.mutator.clone())
                .pipe(pipe)
                .bind(bind)
                .dispatch(count)
                .hub()?;
            match self.control {
                Control::Shape(shapes) => {

                },
                Control::Table(table) => {
                    match table {
                        Table::Array(array) => panic!("not implemented"),
                        Table::Hedge(hedge) => {

                        }
                    }
                }
            }
            // let transform_dispatcher = self
            //     .gpu
            //     .computer()
            //     .mutator(basis_dispatcher)
            return Ok(Hedge {
                buffer: basis,
                mutator,
            }
            .into());
        }
        Err(anyhow!("grid plot not implemented for this shape"))?
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
    Table(Table),
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
