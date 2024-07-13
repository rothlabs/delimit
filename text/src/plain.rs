use serde::Serialize;

use graph::*;

pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod gate;
mod list;

/// to use as Load for higher graph
pub type Role = graph::Role<Actual, Load>;

// type Wow<A> = graph::Role<A, Ploy<Role>>;

/// to use as a Stem (item) for higher graph
pub type View<A> = graph::View<A, Role, Item>; // view::Stem

type Load = Sole<String>;
type Item = PloyView<String, Actual>; // view::Bare
type Text<U> = Pair<U, Load>;

#[derive(Clone, Serialize)]
pub enum Actual {
    List(Text<List>),
    Gate(Text<Gate>),
    Unknown,
}

pub fn string<E>(string: &str) -> View<E> {
    View::Bare(PloyView::Bare(string.into()))
}

pub fn leaf<E>(string: &str) -> View<E> {
    View::Bare(PloyView::Sole(string.leaf()))
}
