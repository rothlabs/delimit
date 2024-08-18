use std::collections::HashMap;
use super::*;

impl Apex {
    pub fn extend(&self, apexes: HashMap<Key, Apex>) -> adapt::Result {
        match self {
            Self::Ploy(ploy) => ploy.adapt(Post::Extend(apexes)),
            _ => Err("No ploy.")?,
        }
    }
}