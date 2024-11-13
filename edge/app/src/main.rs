use graph::*;
use node_derive::*;
use win::*;
use winit::{error::EventLoopError, event_loop::EventLoop};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EventLoopError(#[from] EventLoopError),
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::default();
    let start = Start {
        displays: app.agent.displays.clone(),
    }
    .gate()?;
    start.solve().await?;
    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}

#[derive(GateTag, Back, Debug)]
struct Start {
    displays: Leaf<Vec<Display>>,
}

impl Act for Start {
    async fn act(&self) -> graph::Result<()> {
        let displays = self.displays.base()?;
        Ok(())
    }
}
