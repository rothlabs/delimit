use super::*;
use serde::{Deserialize, Serialize};

/// Runtime-only ID. Used to easily distinguish between hub instances.
pub type Id = u64;

// pub trait ToId {
//     fn id(&self) -> Id;
// }

/// Path component. Used to lookup a hub from another hub such as Lake or Bay.
pub type Key = String;

/// Path to hub. It stands in place of actual hubes in serial form.
#[derive(Clone, PartialEq, Hash, Eq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Path {
    Hash(u64),
    World(Vec<Key>),
    Local(Vec<Key>),
    Upper(Upper<Vec<Key>>),
}

// impl Digest for Path {
//     fn digest<H: Hasher>(&self, state: &mut H) {
//         match self {
//             Self::Hash(x) => x.hash(state),
//             Self::World(x) => x.hash(state),
//             Self::Local(x) => x.hash(state),
//             Self::Upper(x) => x.hash(state),
//         }
//     }
// }

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

impl From<String> for Path {
    fn from(value: String) -> Self {
        Self::Local(vec![value])
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Upper<T> {
    pub rank: usize,
    pub item: T,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub enum Import {
    None,
    World(Stem),
    Local(Node),
    Upper(Upper<Stem>),
}

impl Default for Import {
    fn default() -> Self {
        Self::None
    }
}

pub const WORLD_ALL: Import = Import::World(Stem::All);

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

// pub fn random() -> String {
//     Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
// }
