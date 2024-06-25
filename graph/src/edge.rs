use crate::{base, node};

mod edge;
mod leaf;
mod solver;

pub use edge::Edge;
pub use leaf::Leaf;
pub use solver::Solver;

pub trait CloneUnit {
    type Unit;
    fn unit(&self) -> Self::Unit;
}

pub trait Read {
    type Unit;
    fn read<F: FnOnce(&Self::Unit)>(&self, read: F);
}

pub trait Write {
    type Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, write: F);
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

// pub trait CloneUnit {
//     type Stem: base::Read;
//     fn unit(&self) -> <Self::Stem as base::Read>::Unit;
// }

// pub trait Read {
//     type Stem: base::Read;
//     fn read<F: FnOnce(&<Self::Stem as base::Read>::Unit)>(&self, read: F);
// }

// pub trait Write {
//     type Stem: base::Write;
//     fn write<F: FnOnce(&mut <Self::Stem as base::Write>::Unit)>(&self, write: F);
// }