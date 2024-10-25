pub use plot::*;
pub use shape::*;

use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use wgpu::*;
use bin::*;

pub mod plot;

mod shape;
mod bin;

#[derive(Clone, Debug)]
pub struct Mech {
    gpu: Gpu,
    bin: Grc<Bin>,
}

impl Mech {
    pub fn new(gpu: Gpu) -> graph::Result<Self> {
        Ok(Self {
            bin: Bin::new(&gpu)?.into(),
            gpu,
        })
    }
    pub fn shape(&self, rule: Rule) -> ShapeBuilder {
        ShapeBuilder::default().rule(rule)
    }
    pub fn plot<'a>(&'a self, shape: &'a Shape) -> Plot<'a> {
        Plot {
            bin: &self.bin,
            gpu: &self.gpu,
            shape,
        }
    }
}
