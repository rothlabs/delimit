use graph::*;

pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod gate;
mod list;

pub type View<E> = graph::View<Item, Role, E>;
pub type Role = graph::Role<Load, Exact>;

type Load = Leaf<String>;
type Item = LeafView<String, Exact>;
type Text<U> = Pair<U, Load>;

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
