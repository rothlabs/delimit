use async_trait::async_trait;

use crate::*;

/// The unit to be mutated and a hub back to create backed links.
/// Packs are provided for write-to-unit closures.
pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub struct Out<T> {
    pub roots: Vec<Root>,
    pub id: Id,
    pub out: T,
}

// #[async_trait(?Send)] impl std::future::Future<Output = Result<Vec<Hub<T>>>> + Send
#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait WriteBase<T> {
    /// Front-facing write-to-base.
    async fn write<O: SendSync, F: FnOnce(&mut T) -> O + SendSync>(&self, write: F) -> Result<O>;
    // fn write<O: 'a, F: FnOnce(&mut T) -> O + Send + 'a>(&'a self, write: F) -> impl std::future::Future<Output = Result<O>> + Send + 'a;
    //fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> impl std::future::Future<Output = Result<O>>;
}

pub trait WriteBaseOut<T> {
    /// Write and return the hub meta and graph roots of the rebut. Hub level.
    fn write_base_out<O, F: FnOnce(&mut T) -> O>(&mut self, write: F) -> Result<Out<O>>;
}

#[cfg_attr(not(feature = "oneThread"), async_trait)]
#[cfg_attr(feature = "oneThread", async_trait(?Send))]
pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    async fn write<T: SendSync, F: FnOnce(&mut Pack<Self::Unit>) -> T + SendSync>(
        &self,
        write: F,
    ) -> Result<T>;
}

pub trait WriteUnitOut {
    type Unit;
    /// Write and return the hub meta and graph roots of the rebut.
    /// Takes `&Back` to be included in Pack. Cusp level.
    fn write_unit_out<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
    ) -> Result<Out<T>>;
}

pub trait WriteUnitWork {
    type Unit;
    /// Work-level write-to-unit.
    fn write_unit_work<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> Result<T>;
}
