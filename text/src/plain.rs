use serde::Serialize;

use graph::*;

pub use list::{List, ToList};

#[cfg(test)]
mod tests;

mod list;

/// to use as Load for higher graph
pub type Role = role::Ploy<Actual, Load>;
/// to use as a Stem (item) for higher graph
pub type View<A> = graph::View<role::Ploy<A, Role>, Item>;

type Load = Ace<String>;
type Item = view::Ploy<Actual, String>;
type Text<U> = Deuce<U, Load>;

#[derive(Clone, Serialize)]
pub enum Actual {
    List(Text<List>),
    Unknown,
}

pub fn string<A>(string: &str) -> View<A> {
    View::Base(Item::Base(view::AceView::Bare(string.into())))
}

pub fn leaf<A>(string: &str) -> View<A> {
    View::Base(Item::Base(view::AceView::Ace(string.ace())))
}
