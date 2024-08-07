use serde::{Deserialize, Serialize};

use rand::distributions::{Alphanumeric, DistString};

pub type Id = String;

/// Meta about a Node. It stands in place of actual nodes in serial form.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub id: Id,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            id: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
        }
    }
    pub fn none() -> Self {
        Self { id: "".into() }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ToMeta {
    fn meta(&self) -> Meta;
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
