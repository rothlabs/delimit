use super::*;

pub struct Field {
    pub node: Node,
    pub name: String,
}

impl Field {
    pub fn new(node: Node, name: String) -> Self {
        Self { node, name }
    }
}

// impl Field {
//     fn insert(&self, node: Node) {
//         self.node.insert(&self.name, node);
//     }
// }

// pub struct Field<T> {
//     name: String,
//     link: T,
// }

// pub enum FieldType {
//     String(Field<Value<String>>),
//     // could have special name for Asset<Deuce<T>> because it will be so common. Maybe thats
//     // what asset should be
//     // TextList(Field<Asset<Deuce<List>>>),
// }

// impl<N> FieldEditor<N>
// where
//     N:
// {
//     pub fn insert<L>(&self, node: impl Into<Node<L>>) -> &Self {

//         self
//     }
// }

// pub trait ToField {
//     fn field(&self, name: String) -> FieldEditor;
// }
