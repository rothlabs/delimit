use crate::*;

// pub type Result<T> = std::result::Result<T, anyhow::Error>;

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

pub trait WriteTray<T> {
    /// Front-facing write-to-tray.
    fn write<O, F: FnOnce(&mut T) -> O>(&self, write: F) -> Result<O>;
}

pub trait WriteTrayOut<T> {
    /// Write and return the hub meta and graph roots of the rebut. Hub level.
    fn write_tray_out<O, F: FnOnce(&mut T) -> O>(&mut self, write: F) -> Result<Out<O>>;
}

pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> Result<T>;
}

pub trait WriteUnitOut {
    type Unit;
    /// Write and return the hub meta and graph roots of the rebut.
    /// Takes `&Back` to be included in Pack. Hub level.
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
