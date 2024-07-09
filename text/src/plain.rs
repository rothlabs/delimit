use graph::*;

pub use view::View;
pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod view;
mod gate;
mod list;

pub type Role = graph::Role<Load, Exact>;

type Item = LeafView<String, Exact>;

type Load = Leaf<String>;

type Text<U> = UnitSolver<U, Load>;

#[derive(Clone)]
pub enum Exact {
    List(Text<List>),
    Gate(Text<Gate>),
    Unknown,
}

pub fn string<E>(string: &str) -> View<E> {
    View::Text(LeafView::Bare(string.into()))
}
