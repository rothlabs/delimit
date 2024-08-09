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
// #[serde(rename_all = "PascalCase")]
pub struct Path {
    root: Root,
    pub keys: Vec<Key>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            root: Root::Lake,
            keys: vec![random()],
        }
    }
    pub fn none() -> Self {
        Self {
            root: Root::Lake,
            keys: vec!["".into()],
        }
    }
}

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}

pub fn random() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
enum Root {
    Lake,
    Up(usize),
}

// pub fn id(&self) -> &String {
//     &self.id
// }
// // pub fn string(&self) -> String {
// //     self.id.clone()
// // }

// #[derive(Clone, Hash, PartialEq, Serialize, Deserialize, Debug)]
// pub struct Id(String);

// impl Id {
//     pub fn new() -> Self {
//         Self(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
//     }
//     pub fn none() -> Self {
//         Self("".into())
//     }
//     pub fn string(&self) -> &str {
//         &self.0
//     }
// }

// impl Default for Id {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Eq for Id {}
