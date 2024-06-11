// use std::borrow::Cow;

use serde::Serialize;

use std::sync::Weak;

use crate::Id;

pub trait Node {
    fn id(&self) -> &str;
    fn name(&self) -> &'static str;
}

#[derive(Serialize)]
pub struct Meta {
    id: Id,
    roots: Vec<Root>,
}

impl Meta {
    pub fn new() -> Meta {
        Meta {
            id: Id::new(),
            roots: vec![],
        }
    }
}

pub struct Root(pub Weak<dyn Node>);

impl Serialize for Root {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        if let Some(root) = self.0.upgrade() {
            serializer.serialize_str(root.id())
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
