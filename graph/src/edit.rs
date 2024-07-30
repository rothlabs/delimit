use super::*;

pub struct Field<T> {
    node: Node<T>,
    name: String,
}

impl<T> Field<T> {
    pub fn new(node: Node<T>, name: String) -> Self {
        Self {
            node,
            name,
        }
    }
}

impl<T> Insert<T> for Field<T> 
where 
    Node<T>: Insert<T> 
{
    fn insert(&self, field: &str, node: Node<T>) {
        self.node.insert(field, node);
    }
}

pub trait Insert<T> {
    fn insert(&self, field: &str, node: Node<T>);
}

pub trait InsertMut<T> {
    fn insert_mut(&mut self, field: &str, node: Node<T>);
}




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
