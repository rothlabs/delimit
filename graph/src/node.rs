// use std::borrow::Cow;

use serde::Serialize;

use std::{hash::{Hash, Hasher}, sync::{Arc, RwLock, Weak}};

use crate::Id;

pub struct Node<A: ?Sized> {
    pub content: Arc<RwLock<Box<A>>>,
    pub meta: Meta,
}

impl<A: ?Sized> Node<A> {
    pub fn new(app: Box<A>) -> Self {
        Self {
            content: Arc::new(RwLock::new(app)),
            meta: Meta::new(),
        }
    }
}

impl<A: ?Sized> Clone for Node<A> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            meta: self.meta.clone(),
        }
    }
}

impl<A> PartialEq for Node<A> {
    fn eq(&self, rhs: &Node<A>) -> bool {
        self.meta.id == rhs.meta.id
    }
}

impl<A> Hash for Node<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.meta.id.hash(state);
    }
}

// pub struct Content<T: ?Sized>(Arc<RwLock<T>>);

// impl<T: ?Sized> Clone for Content<T> {
//     fn clone(&self) -> Self {
//         Self (self.0.clone())
//     }
// }

// impl<T: ?Sized> PartialEq for Content<T> {
//     fn eq(&self, other: &Content<T>) -> bool {
//         self.0 == other.0
//     }
// }

#[derive(Clone, PartialEq, Hash)]
pub struct Meta {
    id: Id,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            id: Id::new(),
        }
    }
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

pub trait Nodish {
    fn id(&self) -> &Id;
    fn name(&self) -> &'static str;
}

pub struct Root(pub Weak<dyn Nodish>);

impl Serialize for Root {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        if let Some(root) = self.0.upgrade() {
            serializer.serialize_str(root.id().string())
        } else {
            serializer.serialize_str("")
        }
    }
}


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
