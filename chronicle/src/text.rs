use serde::Serialize;
use graph::{Node, Leaf, Read};

pub mod unit;

pub fn text(unit: impl Unit + 'static) -> Text {
    Text(Node::new(Box::new(unit)))
}

#[derive(Clone, Serialize)]
pub struct Text(
    pub Node<Box<dyn Unit>>
);

impl Text {
    pub fn read(&self) -> Read<Box<dyn Unit>> { 
        self.0.read()
    }
    pub fn string(&self) -> String {
        self.read().leaf().read().clone()
    }
    pub fn json(&self) -> String {
        self.read().json()
    }
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
    fn leaf(&self) -> Node<String>;
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
