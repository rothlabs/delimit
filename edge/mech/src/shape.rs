use std::num::NonZero;

use super::*;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Shape {
    gpu: Gpu,
    #[builder(default = "2")]
    dimension: u8,
    table: Hub<Table>,
    rule: Rule,
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
    pub async fn grid(&self, count: u32) -> graph::Result<Hub<Hedge>> {
        // let table = self.table.base().await?;
        // if let Table::Hedge(hedge) = table {
        //     let buffer = hedge.buffer.base().await?;
        //     let size = buffer.size() * count / 3;
        //     let basis = self.gpu.buffer(size).storage_copy()?;
        if let Rule::Nurbs(order) = self.rule {
            return self.nurbs_grid(order, count).await;
        }
        // }
        Err(anyhow!("grid plot not implemented for this shape"))?
    }
    async fn nurbs_grid(&self, order: u32, count: u32) -> graph::Result<Hub<Hedge>> {
        let shader = self.gpu.shader(include_wgsl!("plot/nurbs_grid.wgsl"));
        // console_log!("count {count}");
        let config = self.gpu.buffer_uniform(&[order, count]);
        let table = self.table.base().await?;
        if let Table::Hedge(hedge) = table {
            let nurbs = hedge.buffer.base().await?;
            let size = nurbs.size() * (count as u64) / 3;
            let basis = self.gpu.buffer(size).storage_copy()?;
            let nurbs_entry = self.gpu.storage(true).entry(0)?.compute()?;
            let basis_entry = self.gpu.storage(false).entry(1)?.compute()?;
            let config_entry = self
                .gpu
                .uniform()
                .min_binding_size(NonZero::new(8).unwrap())
                .entry(2)?
                .compute()?;
            let bind_layout = self
                .gpu
                .bind_layout(&[nurbs_entry, basis_entry, config_entry])
                .make()?;
            let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
            let pipe = shader.compute("main").layout(&pipe_layout).make()?;
            let binder = self
                .gpu
                .binder()
                .layout(bind_layout)
                .entry(0, nurbs)
                .entry(1, basis.clone())
                .entry(2, config)
                .hub()?;
            let mutator = self
                .gpu
                .dispatcher()
                .mutator(hedge.mutator)
                .pipe(pipe)
                .bind(binder)
                .count(count)
                .hub()?;
            return Ok(Hedge {
                buffer: basis.into(),
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
    Table(Hub<Table>),
}

#[derive(Clone, Debug)]
pub struct Instance {
    pub table: Hub<Table>,
    pub layout: Layout,
    pub instance: Option<Box<Instance>>,
}

#[derive(Clone, Debug)]
pub enum Layout {
    Free,
    Grid,
    Radial,
}
