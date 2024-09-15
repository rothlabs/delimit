use async_trait::async_trait;
pub use gain::*;
pub use task::*;

use super::*;
use thiserror::Error;

mod gain;
mod task;

// pub type Result = result::Result<Gain, crate::Error>;

// #[async_trait]
pub trait Solve {
    type Base: 'static + Payload;
    /// Solve a task.
    /// The hub will run computations or return existing results.
    // async fn solve(&self) -> Result<Hub<Self::Base>>;
    #[cfg(not(feature = "oneThread"))]
    fn solve(&self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>> + Send;
    #[cfg(feature = "oneThread")]
    fn solve(&self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>>;
}

pub trait Reckon {
    fn reckon(&self, _: Task) -> Result<Gain> {
        reckon_ok()
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Task(#[from] task::Error),
    #[error(transparent)]
    Gain(#[from] gain::Error),
    #[error(transparent)]
    Aim(#[from] aim::Error),
    #[error(transparent)]
    Hub(#[from] hub::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

pub fn reckon_ok() -> Result<Gain>
{
    Ok(Gain::None)
}

pub fn solve_ok<T>() -> Result<Hub<T>>
where 
    T: 'static + Payload
{
    Ok(Hub::none())
}

// #[async_trait]
pub trait Act {
    /// Perform an external action.
    // async fn act(&self) -> Result<()>;
    #[cfg(not(feature = "oneThread"))]
    fn act(&self) -> impl std::future::Future<Output = Result<()>> + Send;
    #[cfg(feature = "oneThread")]
    fn act(&self) -> impl std::future::Future<Output = Result<()>>;
}

// #[async_trait]
impl<A: Act + SendSync> Solve for A {
    type Base = ();
    async fn solve(&self) -> Result<Hub<()>> {
    //fn solve(&self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>> + Send {
        self.act().await?;
        solve_ok()
    }
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait SolveMut {
    type Base: 'static + Payload;
    /// For graph internals to handle solve calls
    async fn solve(&mut self) -> Result<Hub<Self::Base>>;
    // fn solve<'a>(&'a mut self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>> + 'a;
}

pub trait ReckonMut {
    fn reckon(&mut self, task: Task) -> Result<Gain>;
}



// // #[async_trait]
// pub trait SolveMut {
//     type Base: 'static + Payload;
//     /// For graph internals to handle solve calls
//     // async fn solve(&mut self) -> Result<Hub<Self::Base>>;
//     fn solve(&mut self) -> impl std::future::Future<Output = Result<Hub<Self::Base>>> + Send;
// }