use std::{borrow::Cow, cell::RefCell, rc::Rc};

use serde::{Serializer, Serialize};
use rand::distributions::{Alphanumeric, DistString};

use crate::{app::App, snap::Snap, Id};

#[derive(Clone)]
pub struct Node(pub Option<Rc<RefCell<Junction>>>);

impl Node {
    pub fn none() -> Node {
        Node(None)
    }
    // pub fn new(cast: &'static str) -> Self {
    //     Node(Rc::new(RefCell::new(Some(Junction {
    //         id: Alphanumeric.sample_string(&mut rand::thread_rng(), 16),
    //         cast: Cow::Borrowed(cast),
    //         apps: vec![],
    //         snaps: vec![],
    //     }))))
    // }
}

impl Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(junction) = &self.0 {
            junction.borrow().pair.serialize(serializer)
            //serializer.serialize_str(&junction.borrow().id)
        } else {
            serializer.serialize_str("")
        }
    }
}

#[derive(Serialize)]
pub struct Junction {
    pair: Pair,
    app: App, // called "snap" in old django project
}

#[derive(Serialize)]
struct Pair {
    snap: Snap, // called "version" in old django project
    node: Id,
}

//type Cast = Cow<'static, str>;


//#[derive(Default, Clone, Hash, PartialEq)]
//impl Eq for Junction {}