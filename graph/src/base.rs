use std::sync::{Arc, RwLock, Weak};

use crate::{NO_POISON, REACTOR};

pub trait FromUnit {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
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

pub trait React {
    fn react(&mut self);
}

pub trait AddReactor {
    fn add_reactor(&mut self, reactor: &Reactor);
}

pub trait FromReactor {
    fn from_reactor(&self, reactor: Reactor) -> Self;
}

#[derive(Clone)]
pub struct Reactor(Weak<RwLock<dyn React>>);

impl Reactor {
    pub fn new<R: React + 'static>(reactor: &Arc<RwLock<R>>) -> Self {
        let reactor = Arc::downgrade(&reactor);
        Self(reactor)
    }
    pub fn react(&self) {
        let reactor = self.0.upgrade().expect(REACTOR);
        let mut reactor = reactor.write().expect(NO_POISON);
        reactor.react();
    }
}

// pub struct Reactors(Vec<Reactor>);

// impl Reactors {
//     pub react()
// }