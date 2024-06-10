use std::{cell::RefCell, rc::Rc};

use serde::{Serializer, Serialize};

pub struct App(pub Rc<RefCell<dyn Application>>);

impl Serialize for App {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.borrow().name())
    }
}

pub trait Application {
    fn name(&self) -> &'static str;
}