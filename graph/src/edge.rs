use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};

use serde::{Serializer, Serialize};

use crate::Id;

#[derive(Serialize)]
pub struct Edge<T>(Gate<T>);

impl<T: Clone> Edge<T> {
    fn get(&self, snap: &Id) -> Arc<T> { // TODO: maybe return Arc<dummy> if no key in hashmap?
        match &self.0 {
            Gate::Cold(cold) => cold.0.get(snap).unwrap().clone(),
            Gate::Hot(hot) => {
                let wow = hot.0.lock().expect("mutex should lock").get(snap).cloned();
                if let Some(huh) = wow {
                    *huh.lock().unwrap()
                } else {
                    0
                }
                //wow
            },
        }
    }
    fn get_mut(&self, snap: &Id) -> MutexGuard<T> {
        
    }
}

pub enum Gate<T> {
    Cold(Cold<T>),
    Hot(Hot<T>),
}

pub struct Cold<T>(pub HashMap<Id, Arc<T>>);

pub struct Hot<T>(pub Mutex<HashMap<Id, Arc<Mutex<T>>>>);

impl<T> Serialize for Hot<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
                
    }
}



// pub struct Lead(pub Rc<RefCell<dyn Application>>);

// impl Serialize for Lead {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(self.0.borrow().name())
//     }
// }

// pub trait Application {
//     fn name(&self) -> &'static str;
// }