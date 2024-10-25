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
    bin: MechGrid,
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
            bin: MechGrid {
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
    pub fn plot<'a>(&'a self, shape: &'a Shape) -> Plot<'a> {
        Plot {
            mech: self,
            gpu: &self.gpu,
            shape,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MechGrid {
    basis: MechGridBasis,
}

#[derive(Clone, Debug)]
pub struct MechGridBasis {
    nurbs: Program,
    control: Program,
}

#[derive(Clone, Debug)]
pub struct Program {
    layout: Grc<BindGroupLayout>,
    pipe: Grc<ComputePipeline>,
}
