use std::{cell::{Ref, RefCell}, rc::Rc};

use serde::{Serialize, Serializer};

use crate::Id;

#[derive(Clone, Serialize)]
pub struct App<T> {
    pub value: T,
    pub id: Id,
}

#[derive(Clone)]
pub struct Leaf<T>(pub Rc<RefCell<App<T>>>);

impl<T: ToOwned<Owned = T>> Leaf<T> {
    pub fn get(&self) -> Ref<'_, App<T>> { //  
        self.0.borrow()
    }
    pub fn set(&self, value: &T) {
        self.0.borrow_mut().value = value.to_owned(); 
    }
    // fn serialize(&self) -> String {
    //     serde_json::to_string(self.0.borrow()).unwrap()
    // }
    // pub fn id(&self) -> Id { //  
    //     self.0.as_ref().borrow().id.clone()
    // }
}

impl Leaf<String> {
    pub fn set_str(&self, value: &str) {
        self.0.as_ref().borrow_mut().value = value.to_owned();
    }
}

impl Serialize for Leaf<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().id.serialize(serializer)
        // serializer.serialize_str(&self.get().value)
    }
}

pub fn leaf_str(value: &str) -> Leaf<String> {
    Leaf(Rc::new(RefCell::new(App {
        value: value.to_owned(),
        id: Id::new(),
    })))
}


// pub fn id(&self) -> Ref<'_, Id> { //  impl  Deref<Target = Id> + '_
// Ref::map(self.0.as_ref().borrow(), |r| &r.id)
// }


// impl Node for Leaf<String> {
//     fn string(&self) -> Leaf<String> {
//         self.clone()
//     }
//     fn serialize(&self) -> String {
//         serde_json::to_string(self).unwrap()
//     }
//     fn id(&self) -> Id {
//         self.0.as_ref().borrow().id.clone()
//     }
// }



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