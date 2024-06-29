pub mod edge;
pub mod link;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod work;
pub mod write;

pub use edge::Edge;
pub use link::{Leaf, Link, Solver, Stemmer, ToLeaf};
pub use meta::Meta;
pub use react::{AddReactor, React, Reactor, Reactors, ToReactor, WithReactor};
pub use read::{CloneUnit, Read, Reader, Solve};
pub use repo::Repo;
pub use work::Work;
pub use write::{SolveMut, Write, Writer};

pub trait AddStem {
    type Unit;
    fn add_stem<T, F: FnOnce(&mut Self::Unit, T)>(&mut self, stem: T, add_stem: F);
}

pub trait Clear {
    fn clear(&mut self);
}

pub trait FromUnit {
    type Unit;
    fn new(unit: Self::Unit) -> Self;
}

/// Make a string. ToLeaf comes for free. 
pub trait GraphString {
    fn string(&self) -> String;
}

/// Marker resulting in Read + Write
pub trait Unit {}

pub trait Memory {
    type Load: Clone;
    type Task: Clone;
    fn add(&mut self, task: Self::Task, load: Self::Load);
    fn get(&self, task: &Self::Task) -> Option<&Self::Load>;
}

const NO_POISON: &str = "the lock should not be poisoned";
