use std::sync::{Arc, RwLock};

use crate::{edge, node};

mod leaf;
mod link;
mod reactor;
mod responder;
mod solver;

pub use leaf::Leaf;
pub use link::Link;
pub use reactor::Reactor;
pub use responder::Responder;
pub use solver::Solver;

pub trait Read {
    type Edge: edge::Read;
    fn read<F: FnOnce(&<<Self::Edge as edge::Read>::Stem as node::Read>::Unit)>(&self, read: F);
}

pub trait Write {
    type Edge: edge::Write;
    fn write<F: FnOnce(&mut <<Self::Edge as edge::Write>::Stem as node::Write>::Unit)>(
        &self,
        write: F,
    );
}

pub trait CloneUnit {
    type Edge: edge::CloneUnit;
    fn unit(&self) -> <<Self::Edge as edge::CloneUnit>::Stem as node::Read>::Unit;
}

pub trait Solve {
    type Edge: edge::Solve;
    fn solve(
        &self,
        task: <<Self::Edge as edge::Solve>::Stem as node::Solve>::Task,
    ) -> <<Self::Edge as edge::Solve>::Stem as node::Solve>::Load;
}

pub trait React {
    fn react(&self);
}

pub trait Respond {
    type Edge: edge::Respond;
    fn respond(&self, memo: <<Self::Edge as edge::Respond>::Root as node::Respond>::Memo);
}
