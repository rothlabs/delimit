use std::{cell::RefCell, rc::Rc};

use serde::{Serialize, Serializer};

use crate::Id;

#[derive(Clone)]
pub struct Unit<T> {
    pub at: T,
    pub id: Id,
}

#[derive(Clone)]
pub struct Leaf<T>(
    pub Rc<RefCell<Unit<T>>>
);

impl<T: ToOwned<Owned = T>> Leaf<T> {
    pub fn set(&self, value: &T) {
        self.0.borrow_mut().at = value.to_owned(); 
    }
}

impl Leaf<String> {
    pub fn set_str(&self, value: &str) {
        self.0.as_ref().borrow_mut().at = value.to_owned();
    }
}

impl Serialize for Leaf<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.borrow().at)
    }
}

pub fn leaf_str(value: &str) -> Leaf<String> {
    Leaf(Rc::new(RefCell::new(Unit {
        at: value.to_owned(),
        id: Id::new("leaf/str"),
    })))
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