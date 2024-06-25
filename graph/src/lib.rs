use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

pub mod base;
pub mod meta;
pub mod node;
pub mod edge;
pub mod link;
pub mod repo;
pub mod react;

pub use base::{Work, AddStem, FromUnit, Solve, Clear, Read, Write};
pub use react::{FromReactor, AddReactor, React, Reactor, Reactors};
pub use meta::Meta;
pub use edge::Edge;
pub use link::Link;
pub use repo::Repo;

const NO_POISON: &str = "the lock should not be poisoned";
const ROOT: &str = "there should be a root";
const REACTOR: &str = "there should be a reactor";

#[derive(Clone, Hash, PartialEq, Serialize)]
pub struct Id(pub String);

impl Id {
    pub fn new() -> Id {
        Id(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
    }
    pub fn string(&self) -> &str {
        &self.0
    }
}

impl Eq for Id {}
