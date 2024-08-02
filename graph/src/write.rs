use crate::*;

pub type Result<T> = std::result::Result<T, Error>;

/// The unit to be mutated and a node back to create backed links.
/// Packs are provided for write-to-unit closures.
pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub struct Out<T> {
    pub roots: Vec<Root>,
    pub meta: Meta,
    pub out: T,
}

pub trait WriteLoad {
    type Item;
    /// Front-facing write-to-load.
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> write::Result<T>;
}

pub trait WriteLoadOut {
    type Item;
    /// Write and return the node meta and graph roots of the rebut. Node level.
    fn write_load_out<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> Out<T>;
}

pub trait WriteLoadWork {
    type Item;
    /// Work-level write-to-load.
    fn write_load_work<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T;
}

pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> write::Result<T>;
}

pub trait WriteUnitOut {
    type Unit;
    /// Write and return the node meta and graph roots of the rebut.
    /// Takes `&Back` to be included in Pack. Node level.
    fn write_unit_out<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> Out<T>;
}

pub trait WriteUnitWork {
    type Unit;
    /// Work-level write-to-unit.
    fn write_unit_work<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> T;
}
