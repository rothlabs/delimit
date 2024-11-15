pub use display::Display;
pub use gui::Gui;
pub use winit;

use agent::Agent;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use tokio::task::spawn;
use wgpu::*;
use winit::window::{Window, WindowId};

mod agent;
mod display;
mod gui;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Builder, Back, Debug)]
#[builder(pattern = "owned")]
pub struct Draw {
    stems: Vec<Hub<Mutation>>,
    #[back(skip)]
    window: Grc<Window>,
}

impl Act for Draw {
    async fn act(&self) -> node::Action {
        self.stems.depend().await?;
        self.window.request_redraw();
        acted()
    }
}
