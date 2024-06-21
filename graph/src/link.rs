use crate::{edge, node};

mod link;
mod reactor;
mod back;

pub use link::Link;
pub use reactor::Reactor;

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

pub trait React {
    type Edge: edge::React;
    fn react(&self, vary: <<Self::Edge as edge::React>::Root as node::React>::Vary);
}

// pub trait React {
//     type Root: node::React;
//     fn react(&self, vary: <Self::Root as node::React>::Vary);
// }
