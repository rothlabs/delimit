use std::{cell::{Ref, RefCell}, rc::Rc};

use serde::{Serialize, Serializer};
use graph::{leaf::Leaf, node::Id};

pub mod node;

pub fn text(node: impl Node + 'static) -> Text {
    Text(Rc::new(RefCell::new(node)))
}

#[derive(Clone)]
pub struct Text(pub Rc<RefCell<dyn Node>>);

impl Text {
    pub fn get(&self) -> Ref<'_, dyn Node> { 
        self.0.as_ref().borrow()
    }
    pub fn string(&self) -> String {
        self.get().leaf().get().value.clone()
    }
    pub fn serialize(&self) -> String {
        self.get().serialize()
    }
}

impl Serialize for Text {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.get().id().serialize(serializer)
    }
}

pub trait Node {
    fn id(&self) -> Id;
    fn leaf(&self) -> Leaf<String>;
    fn serialize(&self) -> String;
}





    // pub fn any(&self) -> &dyn Any {
    //     self
    // }

// pub fn leaf(value: &str) -> Text {
//     Text(Rc::new(Leaf {
//         string: string_unit(value),
//         id: Id::new("text/leaf"),
//     }))
// }

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
