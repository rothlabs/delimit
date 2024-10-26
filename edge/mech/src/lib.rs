pub use plot::*;
pub use shape::*;

use bin::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use wgpu::*;

pub mod plot;

mod bin;
mod shape;

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
    pub fn plot(&self, shape: impl Into<Hub<Shape>>) -> Plot {
        Plot {
            mech: self.clone(),
            shape: shape.into(),
        }
    }
}