use std::{any::Any, cell::{Ref, RefCell}, rc::Rc};

use super::text::Text;

pub mod node;
mod tag;
mod attribute;

pub fn html(node: impl Node + 'static) -> Html {
    Html(Rc::new(RefCell::new(node)))
}

#[derive(Clone)]
pub struct Html(pub Rc<RefCell<dyn Node>>);

impl Html {
    pub fn get(&self) -> Ref<'_, dyn Node> {   
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

pub trait Node {
    fn text(&self) -> Text;
}





