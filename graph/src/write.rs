use crate::*;

pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub trait Write {
    type Item;
    fn write<F: FnOnce(&mut Self::Item)>(&mut self, write: F);
}

pub trait Writer {
    type Item;
    fn write<F: FnOnce(&mut Self::Item)>(&self, write: F);
}

pub trait WriteWithBack {
    type Unit;
    fn write_with_back<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, root: &Back);
}

pub trait WriterWithPack {
    type Unit;
    fn write<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F);
}
