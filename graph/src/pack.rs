use std::{sync::{Arc, Mutex}};

use serde::{Serializer, Serialize};

use crate::{snap::Snap, Id};

#[derive(Clone)]
pub struct Pack(pub Arc<Mutex<Package>>);

impl Pack {
    pub fn new() -> Pack {
        Pack(Arc::new(Mutex::new(Package{
            id: "old id".to_string(),
        })))
    }
}

impl Serialize for Pack {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // /self.0.borrow().serialize(serializer)
        serializer.serialize_str(&self.0.lock().unwrap().id)
    }
}

#[derive(Serialize)]
pub struct Package {
    id: Id,
    //snaps: Vec<Snap>,
}