pub use core::*;

use shape::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use star::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::num::NonZero;
use wgpu::*;

pub mod active;
pub mod flat;
pub mod stable;

mod core;
mod shape;
#[cfg(test)]
mod tests;

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
/// The `hedge` field is the literal discrete shape (GPU memory index and command graph).
/// The `shape` field is the continuous shape source.
/// Buffer Layout: position, velocity-by-parameter-1, velocity-by-parameter-2, acceleration-by-parameter-1, acceleration-by-parameter-2, ...
#[derive(Clone, Debug)]
pub struct Chart {
    pub hedge: Hedge,
    pub shape: Hub<flat::Shape>,
}
