use crate::*;
use std::future::Future;

/// The unit to be mutated and a hub back to create backed links.
/// Packs are provided for write-to-unit closures.
pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub trait WriteBase {
    type Base;
    /// Front-facing write-to-base.
    fn write<O, F>(&self, write: F) -> impl Future<Output = Result<O>> + IsSend
    where
        O: IsSend,
        F: FnOnce(&mut Self::Base) -> O + IsSend;
}

pub trait WriteBaseOut {
    type Base;
    /// Write and return the hub meta and graph roots of the rebut. Hub level.
    fn write_base_out<O, F>(&mut self, write: F) -> Result<(Ring, O)>
    where
        F: FnOnce(&mut Self::Base) -> O;
}

pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    fn write<O, F>(&self, write: F) -> impl Future<Output = Result<O>> + IsSend
    where
        O: IsSend,
        F: FnOnce(&mut Pack<Self::Unit>) -> O + IsSend;
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
