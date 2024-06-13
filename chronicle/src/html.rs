use std::{any::Any, cell::{Ref, RefCell}, rc::Rc};

use super::text::Text;

pub mod unit;
mod tag;
mod attribute;

pub fn html(app: impl Unit + 'static) -> Html {
    Html(Rc::new(RefCell::new(app)))
}

#[derive(Clone)]
pub struct Html(pub Rc<RefCell<dyn Unit>>);

impl Html {
    pub fn get(&self) -> Ref<'_, dyn Unit> {   
        self.0.as_ref().borrow()
    }
    pub fn text(&self) -> Text {
        self.get().text()
    }
    // pub fn serialize(&self) -> String {
    //     self.get().serialize()
    // }
    pub fn any(&self) -> &dyn Any {
        self
    }
}

pub trait Unit {
    fn text(&self) -> Text;
}





