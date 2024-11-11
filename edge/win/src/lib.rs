pub use core::App;

use gpu::*;
use graph::*;
use wgpu::*;

mod core;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // #[error(transparent)]
    // SurfaceError(#[from] SurfaceError),
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    // #[error(transparent)]
    // Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}
