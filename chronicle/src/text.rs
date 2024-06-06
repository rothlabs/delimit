use std::rc::Rc;

pub fn list() -> List {
    List::default()
}

pub trait Text {
    fn string(&self) -> String;
}

impl Text for String {
    fn string(&self) -> String {
        self.clone() 
    }
}

#[derive(Default)]
pub struct List {
    pub nodes: Vec<Rc<dyn Text>>,
    pub separator: Option<Rc<dyn Text>>, 
}

impl List {
    pub fn text(self) -> Rc<dyn Text> {
        Rc::new(self)
    }
    pub fn node(&mut self, node: &Rc<dyn Text>) -> &mut Self  {
        self.nodes.push(Rc::clone(node));
        self
    }
    pub fn leaf(&mut self, string: &str) -> &mut Self {
        self.nodes.push(Rc::new(string.to_owned()));
        self
    }
    pub fn list(&mut self, list: List) -> &mut Self  {
        self.nodes.push(Rc::new(list));
        self
    }
    pub fn separator(&mut self, sep: &str) -> &mut Self {
        self.separator = Some(Rc::new(sep.to_owned()));
        self
    }
}

impl Text for List {
    fn string(&self) -> String {
        let list: Vec<String> = self.nodes.iter()
            .map(|stem| stem.string()).collect();
        if let Some(sep) = &self.separator {
            list.join(&sep.string())
        } else {
            list.join("")
        }
    }
}