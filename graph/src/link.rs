use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

pub use leaf::{Leaf, ToLeaf};
pub use solver::Solver;

#[cfg(test)]
mod tests;

mod leaf;
mod solver;

pub trait Stemmer {
    type Unit;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F);
}

pub trait StemSolver {
    type Unit;
    type Load;
    type Task;
    fn stem_solver<T: SolveReact<Self::Task, Self::Load>, F: FnOnce(&mut Self::Unit, Box<dyn SolveReact<Self::Task, Self::Load>>)>(&self, stem: &T, add_stem: F);
}

#[derive(Clone)]
pub struct Link<E> {
    edge: Arc<RwLock<E>>,
    meta: Meta,
}

impl<E> FromUnit for Link<E>
where
    E: FromUnit,
{
    type Unit = E::Unit;
    fn new(unit: E::Unit) -> Self {
        Self {
            edge: Arc::new(RwLock::new(E::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<E> WithReactor for Link<E>
where
    E: WithReactor,
{
    fn with_reactor(&self, reactor: Reactor) -> Self {
        let edge = self.edge.read().expect(NO_POISON);
        Self {
            edge: Arc::new(RwLock::new(edge.with_reactor(reactor))),
            meta: self.meta.clone(),
        }
    }
}

impl<E> ToReactor for Link<E>
where
    E: React + 'static,
{
    fn reactor(&self) -> Reactor {
        let edge = self.edge.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&edge),
            meta: self.meta.clone(),
        }
    }
}

impl<E> Reader for Link<E>
where
    E: Reader + AddReactor + React + 'static,
{
    type Unit = E::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        // TODO: first read and check if it is not added as reactor and then write to do so
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.reader(read);
        let reactor = self.reactor();
        edge.add_reactor(reactor);
    }
}

impl<E> Writer for Link<E>
where
    E: Writer,
{
    type Unit = E::Unit;
    fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        let edge = self.edge.read().expect(NO_POISON);
        edge.writer(write);
    }
}

impl<E> CloneUnit for Link<E>
where
    E: CloneUnit,
{
    type Unit = E::Unit;
    fn unit(&self) -> Self::Unit {
        let edge = self.edge.read().expect(NO_POISON);
        edge.unit()
    }
}

impl<E> Solve for Link<E>
where
    E: Solve,
{
    type Load = E::Load;
    type Task = E::Task;
    fn solve(&self, task: Self::Task) -> Self::Load {
        let edge = self.edge.read().expect(NO_POISON);
        edge.solve(task)
    }
}

impl<E> Stemmer for Link<E>
where
    E: ToReactor + AddStem,
{
    type Unit = E::Unit;
    fn stemmer<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&self, stem: &T, add_stem: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        let reactor_node = edge.reactor(); // make a reactor from edge stem
        let stem = stem.with_reactor(reactor_node); // make a new link with reactor node
        edge.add_stem(stem, add_stem);
    }
}

impl<E> StemSolver for Link<E>
where
    E: ToReactor + AddStem,
    E: Solve,
{
    type Unit = E::Unit;
    // type Load = <E as read::Solve>::Load;
    // type Task = <E as read::Solve>::Task;
    type Load = E::Load;
    type Task = E::Task;
    fn stem_solver<T: SolveReact<Self::Task, Self::Load>, F: FnOnce(&mut Self::Unit, Box<dyn SolveReact<Self::Task, Self::Load>>)>(&self, stem: &T, add_stem: F) {
        let mut edge = self.edge.write().expect(NO_POISON);
        let reactor_node = edge.reactor(); // make a reactor from edge stem
        let stem = stem.solver_with_reactor(reactor_node); // make a new link with reactor node
        edge.add_stem(stem, add_stem); 
    }
}

impl<E> React for Link<E>
where
    E: React,
{
    fn clear(&mut self) -> Reactors {
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.clear()
    }
    fn react(&mut self) {
        let mut edge = self.edge.write().expect(NO_POISON);
        edge.react();
    }
}

impl<E> PartialEq for Link<E> {
    fn eq(&self, other: &Self) -> bool {
        Arc::<RwLock<E>>::ptr_eq(&self.edge, &other.edge) && self.meta == other.meta
    }
}

impl<E> Serialize for Link<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl<E> AddStem for Link<E>
// where
//     E: AddStem, // + React + 'static,
// {
//     type Stem = <E as AddStem>::Stem;
//     fn add_stem(&mut self, stem: Self::Stem) {
//         //let reactor = Reactor::new(&self);
//         // let link = stem.from_reactor(reactor);
//         // let mut stem = self.stem.write().expect(NO_POISON);
//         // stem.add_stem(link);
//         let mut edge = self.edge.write().expect(NO_POISON);
//         edge.add_stem(stem);
//     }
// }

// use crate::*;

// pub trait LinkMeta {
//     fn meta(&self) -> Meta;
// }

// impl<T: LinkMeta> PartialEq for Link<T> {
//     fn eq(&self, other: &Self) -> bool {
//         self.meta() == other.meta()
//     }
// }
