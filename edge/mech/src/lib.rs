pub use core::*;
pub use shape::*;
pub use view::View;
pub use medium::Medium;

use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use star::*;
use wgpu::*;

mod core;
mod shape;
mod view;
mod medium;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

/// Symbolic discrete shape.
/// Contains plots of position and derivatives with respect to shape parameters.
/// The hedge is the GPU command graph for the literal discrete shape.
/// Buffer Layout: position, velocity-by-parameter-1, velocity-by-parameter-2, acceleration-by-parameter-1, acceleration-by-parameter-2, ...
#[derive(Clone, Debug)]
pub struct Chart {
    pub hedge: Hedge,
    pub shape: Hub<Shape>,
}
