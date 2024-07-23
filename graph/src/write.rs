use crate::*;

pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub trait Write {
    type Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> T;
}

pub trait DoWrite {
    type Item;
    fn do_write<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T;
}

pub trait WriteWithPack {
    type Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> T;
}

pub trait WriteWithBack {
    type Unit;
    fn write_with_back<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> T;
}
