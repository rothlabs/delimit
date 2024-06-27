pub mod base;
pub mod edge;
pub mod link;
pub mod make;
pub mod meta;
pub mod node;
pub mod react;
pub mod read;
pub mod repo;
pub mod write;

pub use base::{AddStem, Clear, FromUnit, Memory};
pub use edge::Edge;
pub use link::{Leaf, Link, Solver};
pub use make::{ToLeaf, ToString};
pub use meta::Meta;
pub use node::Pair;
pub use react::{AddReactor, AsReactor, FromReactor, React, Reactor, Reactors};
pub use read::{CloneUnit, Read, Reader, Solve};
pub use write::{Write, Writer, SolveMut};
pub use repo::Repo;

const NO_POISON: &str = "the lock should not be poisoned";

pub trait Unit {}

// const ROOT: &str = "there should be a root";
// const REACTOR: &str = "there should be a reactor";
