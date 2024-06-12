use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard}};

use serde::{Serializer, Serialize};

use crate::{Guard, Id, Node, Nodish, Snap};

//#[derive(Clone)]
pub struct Edge<T: ?Sized> (
    HashMap<Snap, Node<T>>
);

impl<T: ?Sized> Edge<T> {
    pub fn get(&self, snap: &Snap) -> Guard<T> {
        let w1 = self.0.get(snap);
        let w2 = w1.expect("there should be a snap key");
        let w4 = w2.content.read();
        let w5 = w4.expect("the lock should not be poisoned");
        Guard::new(w5)
    }
    // fn write(&self, snap: &Snap) -> RwLockWriteGuard<T> {
    //     self.0.get(snap)
    //         .expect("there should be a snap key")
    //         .write()
    //         .expect("the lock should not be poisoned")
    // }
}

impl<T: ?Sized> Clone for Edge<T> {
    fn clone(&self) -> Self {
        Self (
            self.0.clone(),
        )
    }
}

// impl<T> Serialize for Edge<T> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer {
        
//     }
// }



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