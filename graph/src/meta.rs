use serde::{Deserialize, Serialize};

use rand::distributions::{Alphanumeric, DistString};

pub type Id = String;

pub trait ToId {
    fn id(&self) -> Id;
}

pub type Path = String;

/// Meta about a Node. It stands in place of actual nodes in serial form.
#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
// #[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub path: Path,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            path: random(),
        }
    }
    pub fn none() -> Self {
        Self { path: "".into() }
    }
}

impl Default for Meta {
    fn default() -> Self {
        Self::new()
    }
}

pub fn random() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
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
