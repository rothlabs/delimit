use crate::*;

pub struct Pack<'a, U: 'a> {
    pub unit: &'a mut U,
    pub back: &'a Back,
}

pub type WriteResult<T> = Result<T, String>;

pub trait Write {
    type Item;
    fn write<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> WriteResult<T>;
}

pub trait DoWrite {
    type Item;
    fn do_write<T, F: FnOnce(&mut Self::Item) -> T>(&mut self, write: F) -> T;
}

pub trait WriteWithRoots {
    type Item;
    fn write_with_roots<T, F: FnOnce(&mut Self::Item) -> T>(
        &mut self,
        write: F,
    ) -> (Vec<Root>, Meta, T);
}

pub trait WriteWithPack {
    type Unit;
    fn write<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(&self, write: F) -> WriteResult<T>;
}

pub trait WriteWithBack {
    type Unit;
    fn write_with_back<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> T;
}

pub trait WriteWithBackRoots {
    type Unit;
    fn write_with_back_roots<T, F: FnOnce(&mut Pack<Self::Unit>) -> T>(
        &mut self,
        write: F,
        back: &Back,
    ) -> (Vec<Root>, Meta, T);
}

// pub trait WriteWithRoots {
//     type Item;
//     fn write_with_roots<T, F: FnOnce(&mut Self::Item) -> T>(&self, write: F) -> (Vec<Root>, T);
// }
