use super::*;

#[derive(Clone, Debug)]
pub enum Post {
    Trade(Box<dyn Trade>),
    Import,
    Insert(Node),
    Extend(Vec<Node>),
    Remove(usize),
}

impl Backed for Post {
    fn backed(&self, back: &Back) -> Self {
        match self {
            Post::Insert(nodes) => Post::Insert(nodes.backed(back)),
            _ => self.clone(),
        }
    }
}

// impl Post {
//     // pub fn new() -> Self {
//     //     Self::default()
//     // }
//     // pub fn field(&mut self, field: String) -> &mut Self {
//     //     self.field = field;
//     //     self
//     // }
//     pub fn insert(&mut self, node: impl Into<Node>) -> &mut Self {
//         match &mut self.form {
//             Form::Insert(nodes) => nodes.push(node.into()),
//             _ => self.form = Form::Insert(vec![node.into()]),
//         }
//         self
//     }
//     pub fn extend(&mut self, nodes: Vec<impl Into<Node>>) -> &mut Self {
//         let map = nodes.into_iter().map(|node| node.into());
//         match &mut self.form {
//             Form::Insert(n) => n.extend(map),
//             _ => self.form = Form::Insert(map.collect()),
//         }
//         self
//     }
//     // pub fn import() -> Self {
//     //     Self { field: "".into(), form: Form::Import }
//     // }
// }

// #[derive(Clone)]
// pub enum Form {
//     None,
//     Insert(Vec<Node>),
//     Remove(usize),
//     Import,
// }

// impl Default for Form {
//     fn default() -> Self {
//         Self::None
//     }
// }

// pub fn insert(&mut self, nodes: Vec<Node>) -> &mut Self {
//     self.form = Form::Insert(nodes);
//     self
// }

// pub fn cmd(&mut self, name: &str) -> &mut Self {
//     self.form = Form::Cmd(name.into());
//     self
// }
