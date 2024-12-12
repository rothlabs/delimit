use super::*;

pub mod shape;

/// Continuous parametric geometry.
#[derive(Clone, Debug)]
pub struct Shape {
    pub dimension: u32,
    pub warp: Hedge,
    pub form: shape::Form,
    pub flows: Vec<shape::Flow>,
}

impl Shape {
    pub fn chart<'a>(&'a self, mech: &'a Mech) -> shape::hedge::Chart<'a> {
        shape::hedge::Chart { shape: self, mech }
    }
    pub fn plot_size(&self) -> u32 {
        self.dimension * (self.rank() + 1)
    }
    pub fn rank(&self) -> u32 {
        self.flows.len() as u32
    }
}
