use serde::{Deserialize, Serialize};

use rand::distributions::{Alphanumeric, DistString};

/// Runtime-only ID. Used to easily distinguish between node instances.
pub type Id = String;

pub trait ToId {
    fn id(&self) -> Id;
}

/// Path component. Used to lookup a node from a Bay or Lake.
pub type Key = String;

/// Path to node. It stands in place of actual nodes in serial form.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Path {
    Hash(u64),
    World(Vec<Key>),
    Local(Vec<Key>),
    Upper(Upper),
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Upper {
    rank: usize,
    pub keys: Vec<Key>,
}

pub fn random() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}
