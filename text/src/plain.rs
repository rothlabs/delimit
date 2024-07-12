use serde::Serialize;

use graph::*;

pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod gate;
mod list;

/// to use as Load for higher graph
pub type Role = graph::Role<Load, Exact>;

/// to use as a Stem (item) for higher graph
pub type View<E> = graph::View<Item, Role, E>; // view::Stem

type Load = Sole<String>;
type Item = BaseView<String, Exact>; // view::Bare
type Text<U> = Pair<U, Load>;

#[derive(Clone, Serialize)]
pub enum Exact {
    List(Text<List>),
    Gate(Text<Gate>),
    Unknown,
}

pub fn string<E>(string: &str) -> View<E> {
    View::Item(BaseView::Bare(string.into()))
}

pub fn leaf<E>(string: &str) -> View<E> {
    View::Item(BaseView::Sole(string.leaf()))
}
