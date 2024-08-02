use super::*;
use std::collections::HashMap;

pub struct Repo {
    pub nodes: HashMap<Meta, Node>,
}

impl Repo {
    fn insert(&mut self, nodes: Vec<Node>) {
        for node in nodes {
            let meta = node.meta();
            self.nodes.insert(meta, node);
        }
    }
}

impl Alter for Repo {
    fn alter(&mut self, post: Post, back: &Back) -> alter::Result {
        match post.form {
            Form::Insert(nodes) => (),
            _ => ()
        }
    }
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
