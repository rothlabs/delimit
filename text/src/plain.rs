use graph::*;

pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod list;

pub type Text<U> = UnitSolver<U, Leaf<String>>;

pub enum Item {
    String(String),
    Leaf(Leaf<String>),
    View(View),
}

impl Item {
    fn read<F: FnOnce(&String)>(&self, read: F) {
        match self {
            Item::String(string) => read(string),
            Item::Leaf(leaf) => leaf.reader(read),
            Item::View(view) => view.solver.solve().reader(read),
        };
    }
}

#[derive(Clone)]
pub struct View {
    pub exact: Exact,
    pub solver: link::Solver<Leaf<String>>,
}

impl View {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(reactor),
        }
    }
    pub fn list(text: &Text<List>) -> Self {
        View {
            exact: Exact::List(text.clone()),
            solver: text.solver(),
        }
    }
}

#[derive(Clone)]
pub enum Exact {
    List(Text<List>),
}