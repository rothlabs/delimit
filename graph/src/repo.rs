use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use serde::{Serializer, Serialize};

use crate::{app::App, node::Node, pack::Pack, snap::Snap, user::User, Id};

#[derive(Clone)]
pub struct Repo(pub Arc<Repository>);

impl Repo {
    pub fn new() -> Repo {
        Repo(Arc::new(Repository{
            count: Mutex::new(0),
            packs: HashMap::new(),
        }))
    }
    // pub fn get(&self) {
    //     let r = self.0.count.lock();

    // }
}


pub struct Repository {
    pub count: Mutex<i32>,
    // users: Vec<User>,
    packs: HashMap<Id, Pack>, // called "repo" in old django project
    // snaps: Vec<Snap>, // called "version" in old django project
    // nodes: Vec<Node>,
    // apps: Vec<App>, // called "snap" in old django project
}

// impl Serialize for User {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // /self.0.borrow().serialize(serializer)
//         serializer.serialize_str(&self.0.borrow().id)
//     }
// }
