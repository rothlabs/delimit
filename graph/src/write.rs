use crate::*;

// pub type Result<T> = std::result::Result<T, anyhow::Error>;

/// The unit to be mutated and a apex back to create backed links.
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

pub trait WriteTray {
    /// Front-facing write-to-tray.
    fn write<T, F: FnOnce(&mut Tray) -> T>(&self, write: F) -> Result<T>;
}

pub trait WriteTrayOut {
    /// Write and return the apex meta and graph roots of the rebut. Apex level.
    fn write_tray_out<T, F: FnOnce(&mut Tray) -> T>(
        &mut self,
        write: F,
    ) -> Result<Out<T>>;
}

pub trait WriteUnit {
    type Unit;
    /// Front-facing write-to-unit. Closure takes `Pack { unit, back }`.
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &self,
        write: F,
    ) -> Result<T>;
}

pub trait WriteUnitOut {
    type Unit;
    /// Write and return the apex meta and graph roots of the rebut.
    /// Takes `&Back` to be included in Pack. Apex level.
    fn write_unit_out<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        //back: &Back,
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
