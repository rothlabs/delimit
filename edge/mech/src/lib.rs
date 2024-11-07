pub use plot::*;
pub use shape::*;

use bank::*;
use core::*;
use derive_builder::Builder;
use graph::*;
use gpu::*;
use node_derive::*;
use star::*;
use wgpu::*;

mod bank;
mod core;
mod plot;
mod shape;

pub type Mech = Core;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    GPU(#[from] gpu::Error),
    #[error(transparent)]
    Dom(#[from] dom::Error),
    #[error(transparent)]
    Any(#[from] anyError),
}

pub type Result<T> = std::result::Result<T, Error>;
