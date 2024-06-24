use crate::{base, node};

mod edge;
mod leaf;
mod solver;

pub use edge::Edge;
pub use leaf::Leaf;
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