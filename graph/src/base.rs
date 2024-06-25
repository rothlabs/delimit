use std::sync::{Arc, RwLock, Weak};

use crate::{NO_POISON, REACTOR};

pub trait New {
    fn new() -> Self;
}

pub trait FromUnit {
    type Unit;
    fn from_unit(unit: Self::Unit) -> Self;
}

pub trait Read {
    type Unit;
    fn read(&self) -> &Self::Unit;
}

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&mut self, write: F); 
}

pub trait Work {
    type Task: Clone;
    type Load: Clone;
    fn get(&self, task: &Self::Task) -> Option<Self::Load>;
    fn add(&self, task: Self::Task, load: Self::Load);
}

pub trait Clear {
    fn clear(&self);
}

pub trait AddStem {
    type Stem;
    fn add_stem(&mut self, stem: Self::Stem);
}

pub trait Solve {
    type Task;
    type Load;
    fn solve(&mut self, task: Self::Task) -> Self::Load;
}

pub trait Seek {
    type Root;
    type Task;
    fn root(&self) -> &Self::Root;
    fn task(&self) -> &Self::Task;
}

// pub struct Reactors(Vec<Reactor>);

// impl Reactors {
//     pub react()
// }