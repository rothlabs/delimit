use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

pub use leaf::Leaf;
pub use solver::Solver;

mod leaf;
mod solver;

pub struct Edge<S> {
    pub root: Option<Reactor>,
    pub stem: Arc<RwLock<S>>,
    pub meta: Meta,
}

impl<S> FromUnit for Edge<S>
where
    S: FromUnit,
{
    type Unit = S::Unit;
    fn new(unit: Self::Unit) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(S::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<S> WithReactor for Edge<S> {
    fn with_reactor(&self, reactor: Reactor) -> Self {
        Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

/// make a reactor from the stem for use as the root of another edge
impl<S> ToReactor for Edge<S>
where
    S: React + 'static,
{
    fn reactor(&self) -> Reactor {
        let stem = self.stem.clone() as Arc<RwLock<dyn React>>;
        Reactor {
            item: Arc::downgrade(&stem),
            meta: self.meta.clone(),
        }
    }
}

impl<S> Reader for Edge<S>
where
    S: Read,
{
    type Unit = S::Unit;
    fn reader<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(stem.read());
    }
}

impl<S> CloneUnit for Edge<S>
where
    S: Read,
    S::Unit: Clone,
{
    type Unit = S::Unit;
    fn unit(&self) -> S::Unit {
        let stem = self.stem.read().expect(NO_POISON);
        stem.read().clone()
    }
}

impl<S> Solve for Edge<S>
where
    S: SolveMut,
{
    type Load = S::Load;
    type Task = S::Task;
    fn solve(&self, task: S::Task) -> S::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve_mut(task)
    }
}

impl<S> Writer for Edge<S>
where
    S: Write,
{
    type Unit = S::Unit;
    fn writer<F: FnOnce(&mut S::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
    }
}

impl<S> AddReactor for Edge<S>
where
    S: AddReactor,
{
    fn add_reactor(&mut self, reactor: &Reactor) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.add_reactor(reactor);
    }
}

impl<S> AddStem for Edge<S>
where
    S: AddStem,
{
    type Unit = S::Unit;
    fn add_stem<T, F: FnOnce(&mut S::Unit, T)>(&mut self, stem: T, add_stem: F) {
        let mut edge_stem = self.stem.write().expect(NO_POISON);
        edge_stem.add_stem(stem, add_stem);
    }
}

impl<S> React for Edge<S> {
    fn clear(&mut self) -> Reactors {
        if let Some(root) = &self.root {
            root.clear()
        } else {
            Reactors::default()
        }
    }
    fn react(&mut self) {
        if let Some(root) = &self.root {
            root.react();
        }
    }
}

impl<S> Clone for Edge<S> {
    fn clone(&self) -> Self {
        Self {
            root: None,
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<St> Serialize for Edge<St> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.meta.serialize(serializer)
    }
}

// impl<S> AddStem for Edge<S>
// where
//     S: AddStem, // + React + 'static,
//                 //S::Stem: FromReactor,
// {
//     type Unit = S::Unit;
//     fn add_stem<T: WithReactor, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F) {
//         let mut edge_stem = self.stem.write().expect(NO_POISON);
//         let reactor = edge_stem.reactor();
//         edge_stem.add_stem(stem);
//     }
// }

// impl<S> super::Read for Edge<S>
// where
//     S: Read + AddReactor,
// {
//     type Stem = S;
//     fn read<F: FnOnce(&S::Unit)>(&self, read: F) {
//         let stem = self.stem.read().expect(NO_POISON);
//         read(&stem.read());
//         //stem.add_reactor(&reactor);
//     }
// }

// impl<S> link::React for Edge<S> {
//     fn react(&self) {
//         println!("edge::Edge::react!!!!!");
//         if let Some(root) = &self.root {
//             root.react();
//         }
//     }
// }

// impl<S> Clone for Edge<S> {
//     fn clone(&self) -> Self {
//         Self {
//             stem: self.stem.clone(),
//             root: self.root.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl<S> super::AddLink for Edge<S>
// where
//     S: node::AddLink,
//     S::Link: FromRoot<Root = S>,
// {
//     type Stem = S;
//     fn add_link<F: FnOnce(&mut S::Unit, S::Link)>(&mut self, link: &S::Link, add: F) {
//         let link = link.from_root(&self.stem);
//         let mut stem = self.stem.write().expect(NO_POISON);
//         stem.add_link(link, add);
//     }
// }

// fn write<F: FnOnce(&mut S::Unit) -> R::Vary>(&self, write: F) {
//     let mut stem = self.stem.write().expect(NO_POISON);
//     let variance = write(&mut stem.unit_mut());
//     if let Some(weak) = &self.root {
//         let arc = weak.upgrade().expect(ROOT);
//         let mut root = arc.write().expect(NO_POISON);
//         root.react(variance);
//     }
// }

// impl<S> Edge for Main<S>
// where
//     R: React,
//     S: UnitRef + Solve,
// {
//     type R = R;
//     type S = S;
//     fn new(unit: S::Unit) -> Self {
//         Self {
//             stem: Arc::new(RwLock::new(S::new(unit))),
//             root: None,
//             meta: Meta::new(),
//         }
//     }
//     fn solve(&self, task: S::Task) -> S::Load { // TODO: rename to load?
//         let mut node = self.stem.write().expect(NO_POISON);
//         node.solve(task)
//     }
//     fn unit(&self) -> S::Unit {
//         let node = self.stem.read().expect(NO_POISON);
//         node.unit().clone()
//     }
//     fn read<F: FnOnce(&S::Unit)>(&self, read: F) {
//         let node = self.stem.read().expect(NO_POISON);
//         read(&node.unit());
//     }
//     fn write<F: FnOnce(&mut S::Unit) -> R::Vary>(&self, write: F) {
//         let mut stem = self.stem.write().expect(NO_POISON);
//         let variance = write(&mut stem.unit_mut());
//         if let Some(weak) = &self.root {
//             let arc = weak.upgrade().expect(ROOT);
//             let mut root = arc.write().expect(NO_POISON);
//             root.react(variance);
//         }
//     }
//     fn root(&mut self, stem: &Arc<RwLock<R>>) {
//         self.root = Some(Arc::downgrade(stem));
//     }
// }
