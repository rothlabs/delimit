pub use shape::*;
pub use view::View;

use core::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use star::*;
use wgpu::*;

mod core;
mod shape;
mod view;

pub type Mech = Core;

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

/// Discrete evaluations of a shape.
/// Buffer layout: position, velocity-by-parameter-1, velocity-by-parameter-2, ...
#[derive(Clone, Debug)]
pub struct Plot {
    pub hedge: Hedge,
    pub shape: Hub<Shape>,
}

// #[cfg(target_arch = "wasm32")]
// #[error(transparent)]
// Dom(#[from] dom::Error),
