use super::*;

#[derive(Clone, Default)]
pub struct Post {
    pub field: String,
    pub form: Form,
}

impl Post {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn field(&mut self, field: String) -> &mut Self {
        self.field = field;
        self
    }
    pub fn insert(&mut self, node: impl Into<Node>) -> &mut Self {
        match &mut self.form {
            Form::Insert(nodes) => nodes.push(node.into()),
            _ => self.form = Form::Insert(vec![node.into()]),
        }
        self
    }
}

impl Backed for Post {
    fn backed(&self, back: &Back) -> Self {
        let form = match &self.form {
            Form::Insert(nodes) => Form::Insert(nodes.backed(back)),
            _ => self.form.clone(),
        };
        Self {
            field: self.field.clone(),
            form,
        }
    }
}

#[derive(Clone)]
pub enum Form {
    None,
    Insert(Vec<Node>),
    Remove(usize),
}

impl Default for Form {
    fn default() -> Self {
        Self::None
    }
}

// pub fn insert(&mut self, nodes: Vec<Node>) -> &mut Self {
//     self.form = Form::Insert(nodes);
//     self
// }
