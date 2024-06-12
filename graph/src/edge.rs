use std::{collections::HashMap, hash::Hash, ops::Deref};

use serde::{Serializer, Serialize};

use crate::{Guard, Id, Node, Nodish, Snap};

//#[derive(Clone)]
pub struct Edge<K: Eq + PartialEq + Hash + Clone, A: ?Sized> (
    HashMap<K, Node<A>>
);

impl<K: Eq + PartialEq + Hash + Clone, A> Edge<K, A> {
    pub fn new(key: &K, app: A) -> Self {
        let mut map = HashMap::new();
        map.insert(key.clone(), Node::new(app));
        Self(map)
    }
    pub fn read(&self, key: &K) -> Guard<A> {
        Guard::new(
            self.0.get(key).expect("there should be a matching key")
                .content.read().expect("the lock should not be poisoned")
        )
    }
    // fn write(&self, snap: &Snap) -> RwLockWriteGuard<T> {
    //     self.0.get(snap)
    //         .expect("there should be a snap key")
    //         .write()
    //         .expect("the lock should not be poisoned")
    // }
}

impl<K: Eq + PartialEq + Hash + Clone, A: ?Sized> Clone for Edge<K, A> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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