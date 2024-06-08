use std::{cell::RefCell, rc::Rc};

use serde::{Serialize, Serializer};

use crate::Id;

#[derive(Clone)]
pub struct StringCell<T>{
    pub at: Rc<RefCell<T>>,
    pub id: Id,
}

impl<T: ToOwned<Owned = T>> StringCell<T> {
    pub fn set(&self, value: &T) {
        *self.at.as_ref().borrow_mut() = value.to_owned();
    }
}

impl StringCell<String> {
    pub fn set_str(&self, value: &str) {
        *self.at.as_ref().borrow_mut() = value.to_owned();
    }
}

impl Serialize for StringCell<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.at.borrow())
    }
}

pub fn leaf_str(value: &str) -> StringCell<String> {
    StringCell{
        at: Rc::new(RefCell::new(value.to_owned())),
        id: Id::new("leaf/str"),
    }
}

// #[derive(Clone)]
// pub struct StringCell(pub Rc<RefCell<String>>);

// impl StringCell {
//     pub fn set(&self, value: &str) {
//         *self.0.as_ref().borrow_mut() = value.to_owned();
//     }
// }

// impl Serialize for StringCell {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_str(&self.0.borrow())
//     }
// }

// pub fn string_unit(value: &str) -> StringCell {
//     StringCell(Rc::new(RefCell::new(value.to_owned())))
// }