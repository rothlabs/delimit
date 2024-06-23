use crate::{base, node};

mod edge;
mod leaf;
mod reactor;
mod responder;
mod solver;

pub use edge::Edge;
pub use leaf::Leaf;
pub use reactor::Reactor;
pub use responder::Responder;
pub use solver::Solver;

pub trait Read {
    type Stem: node::Read;
    fn read<F: FnOnce(&<Self::Stem as node::Read>::Unit)>(&self, read: F);
}

pub trait Write {
    type Stem: node::Write;
    fn write<F: FnOnce(&mut <Self::Stem as node::Write>::Unit)>(&self, write: F);
}

pub trait CloneUnit {
    type Stem: node::Read;
    fn unit(&self) -> <Self::Stem as node::Read>::Unit;
}

pub trait Solve {
    type Stem: base::Solve;
    fn solve(&self, task: <Self::Stem as base::Solve>::Task) -> <Self::Stem as base::Solve>::Load;
}

pub trait React {
    fn react(&self);
}

pub trait Respond {
    type Root: node::Respond;
    fn respond(&self, memo: <Self::Root as node::Respond>::Memo);
}

// pub trait AddLink {
//     type Stem: node::AddLink;
//     fn add_link<
//         F: FnOnce(&mut <Self::Stem as node::AddLink>::Unit, <Self::Stem as node::AddLink>::Link),
//     >(
//         &mut self,
//         link: &<Self::Stem as node::AddLink>::Link,
//         add: F,
//     );
// }





// pub trait Solve {
//     type R: Bodied;
//     type S: Bodied;
//     fn new(unit: <Self::S as Bodied>::U) -> Self;
//     fn solve(&self, task: <Self::S as Bodied>::T) -> <Self::S as Bodied>::L;
//     fn unit(&self) -> <Self::S as Bodied>::U;
//     fn read<F: FnOnce(&<Self::S as Bodied>::U)>(&self, read: F);
//     fn write<F: FnOnce(&mut <Self::S as Bodied>::U) -> <Self::R as Bodied>::V>(&self, write: F);
//     fn root(&mut self, stem: &Arc<RwLock<Self::R>>);
// }

// //#[derive(Clone)]
// pub struct BoxAny(pub Box<dyn Any>);

// impl Clone for BoxAny {
//     fn clone(&self) -> Self {
//         Self(
//             Box::new(self.0)
//         )
//     }
// }

// impl Clone for dyn Edge<R = dyn Node<U = dyn Any, T = dyn Any, L = dyn Any, V = dyn Any>, S = dyn Node<U = dyn Any, T = dyn Any, L = dyn Any, V = dyn Any>> {
//     fn clone(&self) -> Self {
//         Self {
//             stem: self.stem.clone(),
//             root: self.root.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

// impl Flatten for String {
//     fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//         flat.units.in
//     }
// }

// impl<U, A, G> PartialEq for Base<U, A, G> {
//     fn eq(&self, rhs: &Base<U, A, G>) -> bool {
//         self.meta.node.id == rhs.meta.node.id
//     }
// }

//clone_trait_object!(Root);
// pub trait Root: { //DynClone {
//     fn clear_work(&mut self);
// }

// impl<U, T, G> Root for Base<U, T, G> {
//     fn clear_work(&mut self) {
//         self.work.clear();
//         for root in self.roots.iter() {
//             if let Some(root) = root.upgrade() {
//                 if let Ok(root) = &mut root.write() {
//                     root.clear_work();
//                 }
//             } // TODO: collect indices of dropped roots to remove from vec (do the same for poisoned?)
//         }
//     }
// }

// pub trait Stem {}
