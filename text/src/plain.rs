use serde::Serialize;

use graph::*;

// pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

// mod gate;
mod list;

/// to use as Load for higher graph
pub type Role = role::Ploy<Actual, Load>;

type PlainPloy<A> = role::Ploy<A, Role>;
/// to use as a Stem (item) for higher graph
pub type View<A> = graph::View<PlainPloy<A>, Item>; // view::Stem

type Load = Sole<String>;
type Item = view::Ploy<Actual, String>;
type Text<U> = Pair<U, Load>;

// TODO: rename to Unit?
#[derive(Clone, Serialize)]
pub enum Actual {
    List(Text<List>),
    // Gate(Text<Gate>),
    Unknown,
}

pub fn string<A>(string: &str) -> View<A> {
    View::Base(Item::Base(BareSole::Bare(string.into())))
}

pub fn leaf<A>(string: &str) -> View<A> {
    View::Base(Item::Base(BareSole::Sole(string.sole())))
}
