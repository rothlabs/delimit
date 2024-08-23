use serde::{Deserialize, Serialize};

use rand::distributions::{Alphanumeric, DistString};

/// Runtime-only ID. Used to easily distinguish between apex instances.
pub type Id = String;

pub trait ToId {
    fn id(&self) -> Id;
}

/// Path component. Used to lookup a apex from another apex such as Lake or Bay.
pub type Key = String;

/// Path to apex. It stands in place of actual apexes in serial form.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Path {
    Hash(u64),
    World(Vec<Key>),
    Local(Vec<Key>),
    Upper(Upper<Vec<Key>>),
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self::Local(vec![value.to_owned()])
    }
}

impl From<&String> for Path {
    fn from(value: &String) -> Self {
        Self::Local(vec![value.clone()])
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Upper<T> {
    pub rank: usize,
    pub item: T,
}

pub fn random() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Import {
    World(Stem),
    Local(Node),
    Upper(Upper<Stem>),
}

pub const WORLD_ALL: Import = Import::World(Stem::All);

// pub fn world_all() -> Import {
//     Import::World(Stem::All)
// }

pub fn upper_all() -> Import {
    Import::Upper(Upper {
        rank: 1,
        item: Stem::All,
    })
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Node {
    key: Key,
    stem: Vec<Stem>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Stem {
    Node(Node),
    All,
}

// impl Import {
//     pub fn upper_all() -> Self {
//         Self::Upper(Upper { rank: 1, item: Stem::All })
//     }
// }
