use super::*;

// #[derive(Clone)]
pub enum Post {
    Insert(Vec<Node>),
    Remove(usize),
}


// pub struct Post {
//     pub back: Back,
//     pub form: Form,
// }

// impl Backed for Post {
//     fn backed(&self, back: &Back) -> Self {
//         Self { 
//             back: back.clone(),
//             form: self.form.clone(),
//         }
//     }
// }

// #[derive(Clone)]
// pub enum Form {
//     Insert(Vec<Node>),
// }
