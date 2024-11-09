// pub use plot::*;
pub use shape::*;

use core::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use star::*;
use wgpu::*;

mod core;
// mod plot;
mod shape;

pub type Mech = Core;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    GPU(#[from] gpu::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[cfg(target_arch = "wasm32")]
    #[error(transparent)]
    Dom(#[from] dom::Error),
}

/// Discrete evaluations of a shape.
/// Data layout: position, velocity-by-parameter-1, velocity-by-parameter-2, ...
#[derive(Clone, Debug)]
pub struct Plot {
    pub hedge: Hedge,
    pub shape: Hub<Shape>,
}
