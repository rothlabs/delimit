// use std::borrow::Cow;

use serde::Serialize;

use std::sync::{Arc, RwLock};

use crate::{Id, Read, Write};

pub struct Node<U: ?Sized> {
    pub unit: Arc<RwLock<U>>,
    pub meta: Meta,
}

impl<U> Node<U> {
    pub fn new(unit: U) -> Self {
        Self {
            unit: Arc::new(RwLock::new(unit)),
            meta: Meta::new(),
        }
    }
    pub fn read(&self) -> Read<U> {
        Read::new(
            self.unit.read().expect("the lock should not be poisoned")
        )
    }
    pub fn write(&self) -> Write<U> {
        Write::new(
            self.unit.write().expect("the lock should not be poisoned")
        )
    }
}

impl<U: ?Sized> Clone for Node<U> {
    fn clone(&self) -> Self {
        Self {
            unit: self.unit.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<U> PartialEq for Node<U> {
    fn eq(&self, rhs: &Node<U>) -> bool {
        self.meta.node.id == rhs.meta.node.id
    }
}

impl<T> Serialize for Node<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.meta.serialize(serializer)
    }
}

#[derive(Clone, Serialize)]
pub struct Meta {
    node: meta::Node,
    //snap: meta::Snap,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            node: meta::Node{id:Id::new()},
            //snap: meta::Snap{}
        }
    }
}

mod meta {
    use std::sync::Weak;

    use serde::Serialize;

    use crate::Id;

    #[derive(Clone, Serialize)]
    pub struct Node {
        pub id: Id,
    } 

    #[derive(Clone)]
    pub struct Snap(Weak<crate::Snap>);
}

    //pub roots: Vec<Root>,

// impl<T: New> Content<T> {
//     pub fn new() -> Content<T> {
//         Content {
//             at: T::new(),
//             id: Id::new(),
//             roots: vec![],
//         }
//     }
// }

// pub struct Root(pub Weak<dyn Nodish>);

// impl Serialize for Root {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         if let Some(root) = self.0.upgrade() {
//             serializer.serialize_str(root.id().string())
//         } else {
//             serializer.serialize_str("")
//         }
//     }
// }


// pub trait Nodish {
//     fn id(&self) -> &Id;
//     fn name(&self) -> &'static str;
// }


// #[derive(Clone, Serialize)]
// pub struct Id {
//     id: crate::Id,
//     cast: Cow<'static, str>,
// }

// impl Id {
//     pub fn new(cast: &'static str) -> Id {
//         Id {
//             id: crate::Id::new(),
//             cast: Cow::Borrowed(cast),
//         }
//     }
// }
