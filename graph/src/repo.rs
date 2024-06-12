use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::{Arc, Mutex}};

use serde::{Serializer, Serialize};

use crate::{edge::Edge, Snap, Id};

// #[derive(Clone)]
// pub struct RepoArcMutex(pub Arc<Mutex<Repo>>);

// #[derive(Clone)]
// pub struct RepoArc(pub Arc<Repo>);

// impl RepoArc {
//     pub fn new() -> RepoArc {
//         RepoArc(Arc::new(Repo {
//             snaps: HashMap::new(),
//             hot_snaps: HashMap::new(),
//         }))
//     }
// }

// //#[derive(Clone)]
// pub struct Repo {
//     snaps: HashMap<Id, SnapWeak>, // called "version" in old django project
//     hot_snaps: HashMap<Id, SnapWeakMutex>,
//     // users: Vec<User>,
//     //pub packs: HashMap<Id, Pack>, // called "repo" in old django project
//     // nodes: Vec<Node>,
//     // apps: Vec<App>, // called "snap" in old django project
// }

// pub struct Repo2 {
//     snaps: HashMap<Id, SnapWeak>, // called "version" in old django project
//     hot_snaps: HashMap<Id, SnapWeakMutex>,
//     // users: Vec<User>,
//     //pub packs: HashMap<Id, Pack>, // called "repo" in old django project
//     // nodes: Vec<Node>,
//     // apps: Vec<App>, // called "snap" in old django project
// }

// impl Repo {
//     pub fn new() -> Repo {
//         Repo {
//             count: Mutex::new(0),
//             packs: HashMap::new(),
//         }
//     }
//     // pub fn get(&self) {
//     //     let r = self.0.count.lock();

//     // }
// }



// impl Serialize for User {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // /self.0.borrow().serialize(serializer)
//         serializer.serialize_str(&self.0.borrow().id)
//     }
// }
