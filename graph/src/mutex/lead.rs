use std::sync::Mutex;

use serde::{Serializer, Serialize};

pub struct Lead<T>(pub Mutex<crate::edge::Edge<T>>);

impl Serialize for Lead<crate::arc::Node> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.lock().unwrap().serialize(serializer)
    }
}