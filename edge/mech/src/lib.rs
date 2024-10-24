pub use plot::*;
pub use shape::*;

use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use wgpu::*;

pub mod plot;

mod shape;

#[derive(Clone, Debug)]
pub struct Mech {
    gpu: Gpu,
    pub programs: ProgramStore,
}

impl Mech {
    pub fn new(gpu: Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/nurbs_grid.wgsl"));
        let setup_entry = gpu.uniform().entry(0)?.compute()?;
        let plan_entry = gpu.storage(true).entry(1)?.compute()?;
        let basis_entry = gpu.storage(false).entry(2)?.compute()?;
        let layout = gpu
            .bind_layout(&[setup_entry, plan_entry, basis_entry])
            .make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let nurbs_grid = Program::new(layout, pipe);
        let programs = ProgramStore {
            nurbs_grid,
            // control_grid,
        };
        Ok(Self {
            gpu,
            programs,
        })
    }
    pub fn shape(&self, rule: Rule) -> ShapeBuilder {
        ShapeBuilder::default().gpu(self.gpu.clone()).mech(self.clone()).rule(rule)
    }
}

#[derive(Clone, Debug)]
pub struct ProgramStore {
    pub nurbs_grid: Program,
    // control_grid: Grc<ComputePipeline>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}

impl Program {
    fn new(layout: Grc<BindGroupLayout>, pipe: Grc<ComputePipeline>) -> Self {
        Self { layout, pipe }
    }
}

// impl Default for ProgramStore {
//     fn default() -> Self {
//         let setup_entry = self.gpu.uniform().entry(0)?.compute()?;
//         let plan_entry = self.gpu.storage(true).entry(1)?.compute()?;
//         let basis_entry = self.gpu.storage(false).entry(2)?.compute()?;
//         let bind_layout = self
//             .gpu
//             .bind_layout(&[setup_entry, plan_entry, basis_entry])
//             .make()?;
//         let pipe_layout = self.gpu.pipe_layout(&[&bind_layout]).make()?;
//         let shader = self.gpu.shader(include_wgsl!("plot/nurbs_grid.wgsl"));
//         let pipe = shader.compute("main").layout(&pipe_layout).make()?;
//         Self {
//             nurbs_grid,
//             control_grid
//         }
//     }
// }

const BASE: u16 = 1;

#[derive(Adapt)]
pub struct Vector {
    units: Vec<Hub<f64>>,
}

impl Solve for Vector {
    type Base = Vec<f64>;
    async fn solve(&self) -> graph::Result<Hub<Vec<f64>>> {
        let mut vector = vec![];
        for unit in &self.units {
            vector.push(unit.base().await?);
        }
        Ok(vector.into_leaf().hub())
    }
    fn rank(&self) -> u16 {
        BASE
    }
}

#[derive(Adapt)]
pub struct Matrix {
    vectors: Vec<Hub<Vec<f64>>>,
}

impl Solve for Matrix {
    type Base = Vec<f64>;
    async fn solve(&self) -> graph::Result<Hub<Vec<f64>>> {
        let mut matrix = vec![];
        for vector in &self.vectors {
            matrix.extend(vector.base().await?);
        }
        Ok(matrix.into_leaf().hub())
    }
    fn rank(&self) -> u16 {
        BASE
    }
}
