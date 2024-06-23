use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

//pub mod leaf;
pub mod base;
pub mod meta;
pub mod node;
//pub mod root;
pub mod edge;
pub mod link;
//pub mod solve;
//pub mod edge;
//pub mod flat;
//pub mod pack;
pub mod repo;
//pub mod snap;
//pub mod roll;
//pub mod user;

//pub use leaf::LeafStr;
pub use base::{FromRoot, FromUnit, Solve, AddLink};
pub use meta::Meta;
//pub use node::ReadWrite;
//pub use root::Root;
pub use edge::Edge;
pub use link::Link;
//pub use solve::SolveReact;
// pub use flat::{Flat, Flatten};
pub use repo::Repo;
// pub use snap::Snap;
// pub use roll::Roll;

const NO_POISON: &str = "the lock should not be poisoned";
const ROOT: &str = "there should be a root";

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
