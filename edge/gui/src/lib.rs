pub use display::Display;
pub use gui::Gui;
pub use winit;

use agent::Agent;
use derive_builder::Builder;
use gfx::Gfx;
use gpu::*;
use graph::*;
use node_derive::*;
use tokio::sync::broadcast;
use tokio::task::{spawn, JoinError};
use wgpu::*;
use winit::window::{Window, WindowId};

mod agent;
mod display;
mod gfx;
mod gui;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Recv(#[from] broadcast::error::RecvError),
    #[error(transparent)]
    Join(#[from] JoinError),
    #[error(transparent)]
    CreateSurface(#[from] CreateSurfaceError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[derive(Clone, Debug)]
enum Post {
    Window(Grc<Window>),
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
