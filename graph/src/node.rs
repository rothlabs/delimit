// use std::borrow::Cow;

use serde::Serialize;

use std::{
    hash::{Hash, Hasher},
    sync::{Arc, RwLock},
};

use crate::{Compute, Flat, Flatten, Id, Read, Write};

// Multiple Nodes can point to the same Unit.
// Pointers to Unit should be serialized as hash digest of Unit.
// Each Unit should be serialized once along side their hash digest.

#[derive(Clone, Serialize)]
pub struct Node<U, G> {
    pub unit: U,
    pub gain: Option<G>,
    pub meta: Meta,
}

impl<U, G> Node<U, G> {
    pub fn new(unit: U) -> Self {
        Self {
            unit, // Arc::new(RwLock::new(unit))
            gain: None,
            meta: Meta::new(),
        }
    }
    // pub fn read(&self) -> Read<U> {
    //     Read::new(self.unit.read().expect("the lock should not be poisoned"))
    // }
    // pub fn write(&self) -> Write<U> {
    //     Write::new(self.unit.write().expect("the lock should not be poisoned"))
    // }
    // pub fn unit_strong_count(&self) -> usize {
    //     Arc::strong_count(&self.unit)
    // }
    // pub fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
    //     let unit = serde_json::to_string(&self.read()).unwrap();
    // }
    // TODO: pub fn duplicate(&self) -> Node<U>  // clone and set new ID
}

// impl Flatten for String {
//     fn flatten(&self, flat: &mut Flat) { // , state: &mut Hasher
//         flat.units.in
//     }
// }

impl Node<String, ()> {
    pub fn str(unit: &str) -> Self {
        Self::new(unit.to_owned())
    }
}

// impl<U> Clone for Node<U> {
//     fn clone(&self) -> Self {
//         Self {
//             unit: self.unit.clone(),
//             meta: self.meta.clone(),
//         }
//     }
// }

impl<U, G> PartialEq for Node<U, G> {
    fn eq(&self, rhs: &Node<U, G>) -> bool {
        self.meta.node.id == rhs.meta.node.id
    }
}

// impl<U: Serialize, G> Serialize for Node<U, G> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         self.meta.serialize(serializer)
//     }
// }

#[derive(Clone, Serialize)]
pub struct Meta {
    pub node: meta::Node,
    //snap: meta::Snap,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            node: meta::Node { id: Id::new() },
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
