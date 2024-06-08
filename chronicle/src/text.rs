use std::{any::Any, rc::Rc};

use serde::{Serialize, Serializer};

use graph::{leaf::StringCell, Id};
pub mod node;
use node::{Node, List};


// pub fn leaf(value: &str) -> Text {
//     Text(Rc::new(Leaf {
//         string: string_unit(value),
//         id: Id::new("text/leaf"),
//     }))
// }

pub fn list() -> List {
    List {
        items: vec![],
        separator: "".into(),
        id: Id::new("text/list"),
    }
}

#[derive(Clone)]
pub struct Text(pub Rc<dyn Node>);
impl Text {
    pub fn string(&self) -> String {
        self.0.string().at.as_ref().borrow().to_owned()
    }
    pub fn serialize(&self) -> String {
        self.0.serialize()
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
        self.0.id().serialize(serializer)
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
