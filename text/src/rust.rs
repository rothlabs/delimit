use graph::*;
use crate::plain::{self, TextList};

pub use struct_rs::StructRs;

#[cfg(test)]
mod tests;

mod struct_rs;

pub type Rust<U> = UnitSolver<U, plain::View>;

enum Item {
    String(String),
    Text(plain::View),
    Rust(View),
}

impl Item {
    fn add_to_list(&self, list: &mut plain::List, reactor: &Reactor) {
        match self {
            Item::String(string) => list.add_str(string),
            Item::Text(view) => list.add_view(view, reactor),
            Item::Rust(view) => list.add_view(&view.solver.solve(), reactor),
        };
    }
}

#[derive(Clone)]
pub struct View {
    pub exact: Exact,
    pub solver: link::Solver<plain::View>,
}

impl View {
    fn with_reactor(&self, reactor: &Reactor) -> Self {
        Self {
            exact: self.exact.clone(),
            solver: self.solver.with_reactor(reactor),
        }
    }
    pub fn struct_rs(rust: &Rust<StructRs>) -> Self {
        View {
            exact: Exact::StructRs(rust.clone()),
            solver: rust.solver(),
        }
    }
}

#[derive(Clone)]
pub enum Exact {
    StructRs(Rust<StructRs>),
}