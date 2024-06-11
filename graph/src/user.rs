use std::{cell::RefCell, rc::Rc};

use serde::{Serializer, Serialize};

// use crate::pack::Pack;

// pub struct User(pub Rc<RefCell<Account>>);

// impl Serialize for User {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         // /self.0.borrow().serialize(serializer)
//         serializer.serialize_str(&self.0.borrow().id)
//     }
// }

// #[derive(Serialize)]
// pub struct Account {
//     id: String,
//     //perms: Vec<Perm>, Permission App
// }

// // #[derive(Serialize)]
// // pub enum Perm {
    
// // }