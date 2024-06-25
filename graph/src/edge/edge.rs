use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::*;

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
    fn from_unit(unit: Self::Unit) -> Self {
        Self {
            root: None,
            stem: Arc::new(RwLock::new(S::from_unit(unit))),
            meta: Meta::new(),
        }
    }
}

impl<S> FromReactor for Edge<S>
// where
//     S: AddReactor,
{
    fn from_reactor(&self, reactor: Reactor) -> Self {
        // let mut stem = self.stem.write().expect(NO_POISON);
        // stem.add_reactor(&reactor);
        Self {
            root: Some(reactor),
            stem: self.stem.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<S> ReadWith for Edge<S>
where
    S: Read,
{
    type Unit = S::Unit;
    fn read<F: FnOnce(&Self::Unit)>(&self, read: F) {
        let stem = self.stem.read().expect(NO_POISON);
        read(&stem.read());
    }
}

impl<S> CloneUnit for Edge<S>
where
    S: Read,
    S::Unit: Clone
{
    type Unit = S::Unit;
    fn unit(&self) -> Self::Unit {
        let stem = self.stem.read().expect(NO_POISON);
        stem.read().clone()
    }
}

impl<S> Solve for Edge<S>
where
    S: Solve,
{
    type Load = S::Load;
    type Task = S::Task;
    fn solve(&self, task: S::Task) -> S::Load {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.solve(task)
    }
}

impl<S> WriteInner for Edge<S>
where
    S: Write,
{
    type Unit = S::Unit;
    fn write<F: FnOnce(&mut S::Unit)>(&self, write: F) {
        let mut stem = self.stem.write().expect(NO_POISON);
        stem.write(write);
        //println!("edge::Edge::write");
        if let Some(root) = &self.root {
            println!("edge::Edge::write -> reactor.react()");
        }
    }
}

impl<S> AddStem for Edge<S>
where
    S: AddStem, // + React + 'static,
    //S::Stem: FromReactor,
{
    type Stem = S::Stem;
    fn add_stem(&mut self, stem: S::Stem) {
        let mut edge_stem = self.stem.write().expect(NO_POISON);
        edge_stem.add_stem(stem);
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
