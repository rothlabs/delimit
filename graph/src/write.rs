use crate::*;
use async_trait::async_trait;

/// The unit to be mutated and a hub back to create backed links.
/// Packs are provided for write-to-unit closures.
pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

// #[async_trait(?Send)] impl std::future::Future<Output = Result<Vec<Hub<T>>>> + Send
#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait WriteBase<T> {
    /// Front-facing write-to-base.
    async fn write<O, F>(&self, write: F) -> Result<O>
    where
        O: SendSync,
        F: FnOnce(&mut T) -> O + SendSync;
    // fn write<O: 'a, F: FnOnce(&mut T) -> O + Send + 'a>(&'a self, write: F) -> impl std::future::Future<Output = Result<O>> + Send + 'a;
    //fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> impl std::future::Future<Output = Result<O>>;
}

pub trait WriteBaseOut<T> {
    /// Write and return the hub meta and graph roots of the rebut. Hub level.
    fn write_base_out<O, F>(&mut self, write: F) -> Result<(Ring, O)>
    where
        F: FnOnce(&mut T) -> O;
}

// #[cfg_attr(not(feature = "oneThread"), async_trait)]
// #[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    fn write<T, F>(&self, write: F) -> impl std::future::Future<Output = Result<T>> + IsSend
    where
        T: SendSync,
        F: FnOnce(&mut Pack<Self::Unit>) -> T + SendSync;
}

pub trait WriteUnitOut {
    type Unit;
    /// Write and return the hub meta and graph roots of the rebut.
    /// Takes `&Back` to be included in Pack.
    fn write_unit_out<O, F>(&mut self, write: F) -> Result<(Ring, O)>
    where
        F: FnOnce(&mut Pack<Self::Unit>) -> O;
}

pub trait WriteUnitWork {
    type Unit;
    /// Work-level write-to-unit.
    fn write_unit_work<T, F>(&mut self, write: F, back: &Back) -> T
    where
        F: FnOnce(&mut Pack<Self::Unit>) -> T;
}
