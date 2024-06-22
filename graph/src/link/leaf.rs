use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::{edge::{self, New}, Meta};

pub struct Leaf<U> {
    edge: Arc<RwLock<edge::Leaf<U>>>,
    meta: Meta,
}

impl<U> self::New for Leaf<U> {
    type Unit = U; 
    fn new(unit: U) -> Self {
        Self {
            edge: Arc::new(RwLock::new(edge::Leaf::new(unit))),
            meta: Meta::new(),
        }
    }
}

impl<U> Clone for Leaf<U> {
    fn clone(&self) -> Self {
        Self {
            edge: self.edge.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U> Serialize for Leaf<U> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.meta.serialize(serializer)
    }   
}

// impl<U> New for Leaf<U> //{
//     where
//         edge::Leaf<U>: edge::New,
//         //U: edge::New,
//     {
//         type Unit = U; //::Unit;
//         fn new(unit: U) -> Self {
//             Self {
//                 edge: Arc::new(RwLock::new(<edge::Leaf<U> as edge::New>::new(unit))),
//                 meta: Meta::new(),
//             }
//         }
//     }

