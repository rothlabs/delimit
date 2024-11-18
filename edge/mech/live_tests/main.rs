use app::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use gui::*;
use mech::*;
use node_derive::*;
use star::*;
use tokio::sync::mpsc;
use winit::{error::EventLoopError, event_loop::EventLoop};

mod app;
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    // let (tx, mut rx) = mpsc::channel::<u32>(32);
    let mut gui = Gui::default();
    let app = AppBuilder::default().displays(&gui.agent.displays).hub()?;
    app.depend().await?;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut gui)?;
    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Mech(#[from] mech::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

// let event_loop = EventLoopBuilder::default().with_any_thread(true).build()?;
