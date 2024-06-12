// use std::borrow::Cow;

use serde::Serialize;

use std::sync::{Arc, Weak, RwLock};

use crate::Id;

// #[derive(Clone)]
// pub struct StrongNode<T>(pub Arc<RwLock<Node<T>>>);

// #[derive(Clone)]
// pub struct WeakNode<T>(pub Weak<RwLock<Node<T>>>);

//#[derive(Clone)]
pub struct Node<T: ?Sized> {
    pub content: Arc<RwLock<T>>,
    pub meta: Meta,
}

impl<T: ?Sized> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            content: self.content.clone(),
            meta: self.meta.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Meta {
    id: Id,
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
