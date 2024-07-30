use super::*;
use std::collections::HashMap;

pub struct Repo<T> {
    pub nodes: HashMap<Meta, Node<T>>,
}

impl<T> InsertMut<T> for Repo<T> {
    fn insert_mut(&mut self, field: &str, node: Node<T>) {
        let meta = node.meta();
        if field == "nodes" {
            self.nodes.insert(meta, node);
        }
    }
}

// impl<T> ToField for Repo<T> {
//     fn field(&self, name: String) -> FieldEditor {
        
//     }
// }
