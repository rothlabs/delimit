use super::*;

enum Item<'a> {
    Text(&'a Text),
    String(&'a str)
}

pub struct List<'a> {
    stems: Vec<Item<'a>>,
    sep: &'a str,
}

impl<'a> List<'a> {
    pub fn make(&self) -> Text {
        let mut list = super::List::default();
        for item in self.stems.iter() {
            match *item {
                Item::String(string) => list.add_str(string),
                Item::Text(text) => list.add_text(text)
            }
        }
        Text { 
            nodes: vec![Rc::new(list)],
        }
    }  
    pub fn string(&mut self, string: &'a str) -> &mut Self {
        self.push(Item::String(string))
    } 
    pub fn text(&mut self, text: &'a Text) -> &mut Self {
        self.push(Item::Text(text))
    }  
    fn push(&mut self, item: Item<'a>) -> &mut Self {
        self.stems.push(item); self
    }
}

pub fn list<'a>() -> List<'a> {
    List {
        stems: vec![],
        sep: "",
    }
}