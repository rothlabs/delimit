pub use encode::post::{compute, render, Command};

use bind::*;
use buffer::*;
use bytemuck::*;
use core::*;
use derive_builder::{Builder, UninitializedFieldError};
use encode::*;
use graph::*;
use node_derive::*;
use pipe::*;
use shader::*;
use star::*;
use std::fmt::Debug;
use util::DeviceExt;
use wgpu::*;
use winit::window::Window;

mod bind;
mod buffer;
mod core;
mod encode;
mod pipe;
mod shader;
mod texture;

pub type Gpu = Core;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SurfaceError(#[from] SurfaceError),
    #[error(transparent)]
    RequestDeviceError(#[from] RequestDeviceError),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
    #[cfg(target_arch = "wasm32")]
    #[error(transparent)]
    Dom(#[from] dom::Error),
}

#[derive(Clone, Debug)]
pub struct Hedge {
    pub buffer: Hub<Grc<Buffer>>,
    pub root: Hub<Mutation>,
}

#[derive(Clone, Default, Debug)]
pub struct Mutation;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Builder, Back, Debug)]
#[builder(pattern = "owned")]
pub struct Draw {
    stems: Vec<Hub<Mutation>>,
    #[back(skip)]
    window: Grc<Window>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Act for Draw {
    async fn act(&self) -> graph::Result<()> {
        self.stems.depend().await?;
        self.window.request_redraw();
        Ok(())
    }
}
