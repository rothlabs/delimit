use node::TradeNodes;

use super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    fn trade(&mut self, trade: &dyn Trade) -> adapt::Result {
        self.items = self.items.trade(trade);
        self.separator = self.separator.trade(trade);
        Ok(Gain::None)
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
    fn stems(&self) -> solve::Result {
        let mut nodes = self.items.clone();
        nodes.push(self.separator.clone());
        Ok(nodes.tray())
    }
}

impl Adapt for List {
    fn adapt(&mut self, post: Post) -> adapt::Result {
        match post {
            Post::Trade(trade) => self.trade(trade.as_ref()),
            _ => did_not_adapt(post),
        }
    }
}

impl Solve for List {
    fn solve(&self, task: Task) -> solve::Result {
        match task {
            Task::Main => self.main(),
            Task::Stems => self.stems(),
            _ => did_not_solve(),
        }
    }
}

// impl Make for List {
//     fn make(&self, back: &Back) -> Self {
//         Self {
//             items: self.items.backed(back),
//             separator: self.separator.backed(back),
//         }
//     }
// }

// fn swap<F: Fn(&Node) -> Node>(&mut self, swap: F) -> adapt::Result {

// impl Stems for List {
//     fn stems(&self) -> Vec<&Node> {
//         let mut stems: Vec<&Node> = self.items.iter().collect();
//         stems.push(&self.separator);
//         stems
//     }
// }

// ///////////////////////////////////
// /// instead of Make and SerializeGraph for unit
// ///     impl Stem or Stems that returns Vec<&mut Node>
// ///     that should cover the creation process
// ///     and serialization
// /// //////////////////////////

// impl Stems for List {
//     fn stems(&mut self) -> Vec<&mut Node> {
//         let mut stems: Vec<&mut Node> = self.items.iter_mut().collect();
//         stems.push(&mut self.separator);
//         stems
//     }
// }
