use super::*;
use std::collections::HashMap;

pub struct Repo {
    pub nodes: HashMap<Meta, Node>,
}

// impl InsertMut for Repo {
//     fn insert_mut(&mut self, field: &str, node: Node) {
//         let meta = node.meta();
//         if field == "nodes" {
//             self.nodes.insert(meta, node);
//         }
//     }
// }

// impl<T> ToField for Repo<T> {
//     fn field(&self, name: String) -> FieldEditor {

//     }
// }
