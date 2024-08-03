use serde::Serialize;

use super::*;

#[derive(Default, Serialize)]
pub struct List {
    items: Vec<Node>,
    separator: Node,
}

impl List {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn separator(&mut self, separator: impl Into<Node>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(&mut self, items: Vec<impl Into<Node>>) -> &mut Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn push(&mut self, item: impl Into<Node>) -> &mut Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
    fn serial(&self, serial: &mut Serial) -> solve::Result {
        self.separator.serial(serial)?;
        for item in &self.items {
            item.serial(serial)?;
        }
        Ok(Tray::None)
    }
    fn main(&self) -> solve::Result {
        if self.items.is_empty() {
            return Node::empty();
        }
        let last = self.items.len() - 1;
        let mut string = String::new();
        self.separator.read_string(|sep| {
            for i in 0..last {
                self.items[i].read_string(|s| string += s);
                string += sep;
            }
        });
        self.items[last].read_string(|s| string += s);
        Ok(string.leaf().node().tray())
    }
}

impl Stems for List {
    fn stems(&self) -> Vec<&Node> {
        let mut stems: Vec<&Node> = self.items.iter().collect();
        stems.push(&self.separator);
        stems
    }
}

///////////////////////////////////
/// instead of Make and SerializeGraph for unit
///     impl Stem or Stems that returns Vec<&mut Node>
///     that should cover the creation process 
///     and serialization
/// //////////////////////////
impl Make for List {
    fn make(&self, back: &Back) -> Self {
        Self {
            items: self.items.backed(back),
            separator: self.separator.backed(back),
        }
    }
}

impl Solve for List {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Node => self.main(),
            Task::Serial(serial) => self.serial(serial),
            _ => Ok(Tray::None)
        }
    }
}

impl Alter for List {
    fn alter(&mut self, _: Post) -> alter::Result {
        Ok(Report::None)
    }
}


// impl Stems for List {
//     fn stems(&mut self) -> Vec<&mut Node> {
//         let mut stems: Vec<&mut Node> = self.items.iter_mut().collect();
//         stems.push(&mut self.separator);
//         stems
//     }
// }