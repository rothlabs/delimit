use serde::Serialize;

use crate::Id;

#[derive(Clone, Serialize)]
pub struct Meta {
    pub id: Id,
}

impl Meta {
    pub fn new() -> Self {
        Self { id: Id::new() }
    }
}
