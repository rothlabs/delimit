use derive_builder::Builder;
use gpu::*;
use graph::*;
use node_derive::*;
use win::*;
use winit::{error::EventLoopError, event_loop::EventLoop};
use app::*;

mod app;

pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let mut gui = Gui::default();
    let test = AppBuilder::default().displays(&gui.agent.displays).hub()?;
    test.depend().await?;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut gui).unwrap();
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}