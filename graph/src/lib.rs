pub mod base;
pub mod meta;
pub mod node;
pub mod edge;
pub mod link;
pub mod repo;
pub mod read;
pub mod write;
pub mod react;

pub use base::{Work, AddStem, FromUnit, Clear};
pub use read::{Read, ReadWith, CloneUnit, Solve};
pub use write::{Write, WriteInner};
pub use react::{FromReactor, AddReactor, React, Reactor, Reactors};
pub use meta::Meta;
pub use edge::Edge;
pub use link::Link;
pub use repo::Repo;

const NO_POISON: &str = "the lock should not be poisoned";
// const ROOT: &str = "there should be a root";
// const REACTOR: &str = "there should be a reactor";
