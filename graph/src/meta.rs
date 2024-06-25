use serde::Serialize;

use rand::distributions::{Alphanumeric, DistString};

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

#[derive(Clone, Serialize)]
pub struct Meta {
    pub id: Id,
}

impl Meta {
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}
