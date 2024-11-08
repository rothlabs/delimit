use buffer::*;
use bytemuck::*;
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

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    // TODO: can uninitialized field error be removed because it is in graph crate?
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Dom(#[from] dom::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
pub struct Hedge {
    pub buffer: Hub<Grc<Buffer>>,
    pub root: Hub<Mutation>,
}

#[derive(Clone, Default, Debug)]
pub struct Mutation;