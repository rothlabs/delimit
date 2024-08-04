use serde::{Deserialize, Serialize};

use rand::distributions::{Alphanumeric, DistString};

#[derive(Clone, Hash, PartialEq, Serialize, Deserialize, Debug)]
pub struct Id(String);

impl Id {
    pub fn new() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::thread_rng(), 16))
    }
    pub fn none() -> Self {
        Self("".into())
    }
    pub fn string(&self) -> &str {
        &self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Eq for Id {}

/// Meta about a Node. It stands in place of actual nodes in serial form.
#[derive(Clone, PartialEq, Eq, Serialize, Debug, Hash)]
pub struct Meta {
    pub id: Id,
}

impl Meta {
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
    pub fn none() -> Self {
        Self { id: Id::none() }
    }
    pub fn id(&self) -> &String {
        &self.id.0
    }
    pub fn string(&self) -> String {
        self.id.0.clone()
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
