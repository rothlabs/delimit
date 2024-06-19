use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

pub mod edge;
pub mod flat;
pub mod leaf;
pub mod node;
pub mod root;
pub mod pack;
pub mod repo;
pub mod snap;
pub mod solve;
pub mod swap;
pub mod user;

pub use edge::Edge;
pub use flat::{Flat, Flatten};
pub use leaf::LeafStr;
pub use node::{Base, Stem};
//pub use node::Root;
pub use repo::Repo;
pub use snap::Snap;
pub use solve::Solve;
pub use swap::Swap;

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
