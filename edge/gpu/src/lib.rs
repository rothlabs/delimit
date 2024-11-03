pub use buffer::*;
pub use bytemuck::*;
pub use display::Display;
pub use flume;
pub use wgpu::{include_wgsl, BufferUsages};

use bind::*;
use core::*;
use derive_builder::{Builder, UninitializedFieldError};
use encode::*;
use graph::*;
use node_derive::*;
use pipe::*;
use shader::*;
use star::*;
use std::fmt::Debug;
use texture::*;
use util::DeviceExt;
use web_sys::HtmlCanvasElement;
use wgpu::*;

mod bind;
mod buffer;
mod core;
mod display;
mod encode;
mod pipe;
mod shader;
mod texture;

pub type Gpu = Core;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Hedge {
    pub buffer: Hub<Grc<Buffer>>,
    pub root: Hub<Mutation>,
}

#[derive(Clone, Default, Debug)]
pub struct Mutation;

// pub struct JoinMutation {

// }

// #[derive(Clone, Debug)]
// pub struct Drawing;

// #[derive(Clone, Debug)]
// pub enum Table {
//     Hedge(Hedge),
//     Array(Hub<Vec<f64>>),
// }

// impl Hedge {
//     pub async fn new<T>(gpu: Gpu, data: impl Into<Hub<Vec<T>>>) -> graph::Result<Self>
//     where
//         T: Pod + Debug,
//     {
//         let data = data.into();
//         let size = data.base().await?.len() as u64 * 4;
//         let buffer: Hub<Grc<Buffer>> = gpu.buffer(size).storage_copy()?.into();
//         let root = gpu.writer(buffer.clone()).data(data).hub()?;
//         Ok(Self { buffer, root })
//     }
// }

// #[derive(ThisError, Debug)]
// pub enum Error {
//     #[error(transparent)]
//     Graph(#[from] graph::Error),
//     #[error(transparent)]
//     CreateSurfaceError(#[from] CreateSurfaceError),
//     #[error(transparent)]
//     Uninit(#[from] UninitializedFieldError),
//     #[error(transparent)]
//     BufferAsync(#[from] BufferAsyncError),
//     #[error(transparent)]
//     Any(#[from] anyError),
// }
