use rand::distributions::{Alphanumeric, DistString};
use serde::Serialize;

pub mod leaf;
pub mod meta;
pub mod node;
pub mod root;
pub mod solve;
pub mod stem;
//pub mod edge;
//pub mod flat;
//pub mod pack;
pub mod repo;
//pub mod snap;
//pub mod roll;
//pub mod user;

pub use leaf::LeafStr;
pub use meta::Meta;
pub use node::Node;
pub use root::Root;
pub use solve::Solve;
pub use stem::Stem;
// pub use flat::{Flat, Flatten};
pub use repo::Repo;
// pub use snap::Snap;
// pub use roll::Roll;

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
