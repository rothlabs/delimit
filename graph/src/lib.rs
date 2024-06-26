pub mod base;
pub mod meta;
pub mod node;
pub mod edge;
pub mod link;
pub mod repo;
pub mod read;
pub mod write;
pub mod react;
pub mod make;

pub use base::{Work, AddStem, FromUnit, FromUnit2, Clear};
pub use read::{Read, ReadWith, CloneUnit, Solve};
pub use write::{Write, Writer};
pub use react::{FromReactor, AsReactor, AddReactor, React, Reactor, Reactors};
pub use make::{ToString, ToLeaf};
pub use node::{
    //Leaf,
    Pair,
};
pub use meta::Meta;
pub use edge::Edge;
pub use link::{Link, Solver, Leaf};
pub use repo::Repo;

const NO_POISON: &str = "the lock should not be poisoned";
// const ROOT: &str = "there should be a root";
// const REACTOR: &str = "there should be a reactor";
