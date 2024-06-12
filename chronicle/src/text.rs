use std::{cell::{Ref, RefCell}, rc::Rc, sync::{Arc, RwLockReadGuard}};

use serde::{Serialize, Serializer};
use graph::{Edge, Snap, Guard};
use graph::Leaf;
//use graph;

pub mod node;

// pub fn text(app: impl TextContent + 'static) -> Text {
//     Text(Rc::new(RefCell::new(app)))
// }

#[derive(Clone)]
pub struct Text(
    pub Edge<dyn Unit>
);

impl Text {
    pub fn get(&self, snap: &Snap) -> Guard<dyn Unit> { 
        self.0.get(snap)
    }
    // pub fn string(&self) -> String {
    //     self.get().leaf().get().value.clone()
    // }
    // pub fn json(&self) -> String {
    //     self.get().json()
    // }
}

// impl Serialize for Text {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         self.get().node().serialize(serializer)
//     }
// }

//pub trait NewTrait: Node + Clone {}

pub trait Unit {
    //fn node(&self) -> Node;
    fn leaf(&self) -> Leaf<String>;
    fn json(&self) -> String;
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
