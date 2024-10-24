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
    pub grid: MechGrid,
}

impl Mech {
    pub fn new(gpu: Gpu) -> graph::Result<Self> {
        let shader = gpu.shader(include_wgsl!("plot/grid/basis/nurbs.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let span = gpu.bind_storage(true).entry(1)?.compute()?;
        let basis = gpu.bind_storage(false).entry(2)?.compute()?;
        let layout = gpu.bind_layout(&[rig, span, basis]).make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let grid_basis_nurbs = Program { layout, pipe };

        let shader = gpu.shader(include_wgsl!("plot/grid/basis/control.wgsl"));
        let rig = gpu.bind_uniform().entry(0)?.compute()?;
        let basis = gpu.bind_storage(true).entry(1)?.compute()?;
        let index = gpu.bind_storage(true).entry(2)?.compute()?;
        let control = gpu.bind_storage(true).entry(3)?.compute()?;
        let plot = gpu.bind_storage(false).entry(4)?.compute()?;
        let layout = gpu
            .bind_layout(&[rig, basis, index, control, plot])
            .make()?;
        let pipe_layout = gpu.pipe_layout(&[&layout]).make()?;
        let pipe = shader.compute("main").layout(&pipe_layout).make()?;
        let grid_basis_control = Program { layout, pipe };

        Ok(Self {
            gpu,
            grid: MechGrid {
                basis: MechGridBasis {
                    nurbs: grid_basis_nurbs,
                    control: grid_basis_control,
                },
            },
        })
    }
    pub fn shape(&self, rule: Rule) -> ShapeBuilder {
        ShapeBuilder::default()
            .gpu(self.gpu.clone())
            .mech(self.clone())
            .rule(rule)
    }
}

#[derive(Clone, Debug)]
pub struct MechGrid {
    pub basis: MechGridBasis,
}

#[derive(Clone, Debug)]
pub struct MechGridBasis {
    pub nurbs: Program,
    pub control: Program,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub layout: Grc<BindGroupLayout>,
    pub pipe: Grc<ComputePipeline>,
}

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

// let nurbs_grid = Program::new(layout, pipe);
//         let programs = ProgramStore {
//             nurbs_grid,
//             // control_grid,
//         };

// #[derive(Clone, Debug)]
// pub struct ProgramStore {
//     pub nurbs_grid: Program,
//     // control_grid: Grc<ComputePipeline>,
// }

// #[derive(Clone, Debug)]
// pub struct Program {
//     pub layout: Grc<BindGroupLayout>,
//     pub pipe: Grc<ComputePipeline>,
// }

// impl Program {
//     fn new(layout: Grc<BindGroupLayout>, pipe: Grc<ComputePipeline>) -> Self {
//         Self { layout, pipe }
//     }
// }
