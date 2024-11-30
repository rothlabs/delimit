use super::*;
use std::future::Future;

pub type Result<T> = std::result::Result<Hub<T>, Box<dyn std::error::Error + Send + Sync>>;
pub type Action = Result<()>;

pub trait Solve {
    type Base: 'static + SendSync;
    /// Solve a task.
    /// The node will run computations or return existing results.
    fn solve(&self) -> impl Future<Output = node::Result<Self::Base>> + IsSend;
    fn rank(&self) -> u16 {
        0
    }
}

pub trait Act {
    fn act(&self) -> impl Future<Output = node::Action> + IsSend;
}

impl<T: Act + SendSync> Solve for T {
    type Base = ();
    async fn solve(&self) -> node::Action {
        self.act().await?;
        acted()
    }
}

pub fn acted<T>() -> node::Result<T> {
    Ok(Hub::none())
}

pub trait Adapt {
    fn adapt(&mut self, _: &mut dyn Deal) -> crate::Result<()> {
        Err(anyhow!("Adapt::adapt not implemented"))?
    }
    fn back(&mut self, _: &Back);
}
