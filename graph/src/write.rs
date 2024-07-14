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
    fn writer<F: FnOnce(&mut Self::Item)>(&self, write: F);
}

pub trait WriteWithBack {
    type Unit;
    fn write_with_back<F: FnOnce(&mut Pack<Self::Unit>)>(&mut self, write: F, root: &Back);
}

pub trait WriterWithPack {
    type Unit;
    fn writer<F: FnOnce(&mut Pack<Self::Unit>)>(&self, write: F);
}

pub trait Grantor {
    type Load;
    fn grantor(&mut self) -> Self::Load;
}

pub trait Solver {
    type Task;
    type Load;
    fn solver(&mut self, task: Self::Task) -> Self::Load;
}
