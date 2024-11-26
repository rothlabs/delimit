use app::*;
use derive_builder::Builder;
use gpu::*;
use graph::*;
use gui::*;
use mech::*;
use node_derive::*;
use star::*;
use winit::{error::EventLoopError, event_loop::EventLoop};

mod app;
mod tests;

#[tokio::main]
async fn main() -> Result<()> {
    let mut gui = Gui::default();
    let app = app().displays(&gui.gfx.displays).hub()?;
    app.depend().await?;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut gui)?;
    Ok(())
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EventLoop(#[from] EventLoopError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error(transparent)]
    Mech(#[from] mech::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

// might need this to run test on different thread
// let event_loop = EventLoopBuilder::default().with_any_thread(true).build()?;
