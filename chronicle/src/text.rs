use std::{any::Any, cell::RefCell, rc::Rc};

use serde::{Serialize, Serializer};

use graph::Id;
pub mod node;
use node::{Node, List};


// pub fn leaf(value: &str) -> Text {
//     Text(Rc::new(Leaf {
//         string: string_unit(value),
//         id: Id::new("text/leaf"),
//     }))
// }



#[derive(Clone)]
pub struct Text(pub Rc<RefCell<dyn Node>>);
impl Text {
    pub fn string(&self) -> String {
        self.0.borrow().string().0.borrow().at.to_owned()
    }
    pub fn serialize(&self) -> String {
        self.0.borrow().serialize()
    }
    pub fn any(&self) -> &dyn Any {
        self
    }
}
impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.borrow().id().serialize(serializer)
    }
}

// impl RcText {
//     fn new(value: dyn Text) -> RcText {
//         RcText(Rc::new(value))
//     }
// }

//trait TextNode: Text + MutGraph {}

// impl Node for Leaf {
//     fn save(&mut self, graph: &mut Graph) {
//         self.id = graph.save(&self.id, &|unit| {
//             unit.string(&LEAF, &self.string);
//         });
//     }
// }

// impl Node for List {
//     fn save(&mut self, graph: &mut Graph) {
//         self.id = graph.save(&self.id, &|unit| {
//             unit.cast("chronicle/list")
//                 .string("separator", &self.separator)
//                 .nodes(graph, "items", self.stems[0].as_ref());
//         });
//     }
// }
