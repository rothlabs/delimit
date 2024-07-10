use graph::*;

pub use gate::{Gate, TextGate};
pub use list::{List, TextList};
pub use view::View;

#[cfg(test)]
mod tests;

mod gate;
mod list;
mod view;

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
    View::Item(LeafView::Bare(string.into()))
}

pub fn leaf<E>(string: &str) -> View<E> {
    View::Item(LeafView::Leaf(string.leaf()))
}
