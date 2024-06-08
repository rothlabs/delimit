use std::rc::Rc;

use graph::Id;
use serde::Serialize;

pub fn list() -> List {
    List::default()
}

pub trait Text {
    fn string(&self) -> RcString;
    fn id(&self) -> &Id;
}

#[derive(Serialize)]
pub struct Leaf {
    string: RcString,
    id: Id,
}

impl Text for Leaf {
    fn string(&self) -> RcString {
        self.string.clone()
    }
    fn id(&self) -> &Id {
        &self.id
    }
}

pub fn leaf(string: &str) -> RcText {
    RcText(Rc::new(Leaf{
        string: RcString(Rc::new(string.to_owned())),
        id: Id::new(),
    }))
}

#[derive(Default, Serialize)]
pub struct List {
    pub items: Vec<RcText>,
    pub separator: String,
    pub id: Id,
}

impl List {
    pub fn text(self) -> RcText {
        RcText(Rc::new(self))
    }
    pub fn node(&mut self, node: &RcText) -> &mut Self {
        self.items.push(node.clone());
        self
    }
    pub fn leaf(&mut self, string: &str) -> &mut Self {
        self.items.push(leaf(string));
        self
    }
    pub fn list(&mut self, list: List) -> &mut Self {
        self.items.push(RcText(Rc::new(list)));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = sep.to_owned();
        self
    }
}

impl Text for List {
    fn string(&self) -> RcString {
        let strings: Vec<RcString> = self.items.iter().map(|i| i.0.string()).collect();
        let list: Vec<&str> = strings.iter().map(|s| s.0.as_str()).collect();
        RcString(Rc::new(list.join(&self.separator)))
    }
    fn id(&self) -> &Id {
        &self.id
    }
}

#[derive(Clone)]
pub struct RcString(pub Rc<String>);
impl Serialize for RcString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_str(self.0.as_ref())
    }
}

#[derive(Clone)]
pub struct RcText(pub Rc<dyn Text>);
impl RcText {
    pub fn string(&self) -> String {
        self.0.string().0.as_ref().to_owned()
    }
}
impl Serialize for RcText {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
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
