use serde::Serialize;

use graph::*;

pub use list::{List, ToList};

#[cfg(test)]
mod tests;

mod list;

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

/// Plain text to use inside units of super graphs
pub type View<A> = graph::View<role::Ploy<A, Role>, Item>;

type Load = Ace<String>;
type Item = view::Ploy<Part, String>;
type Text<U> = Deuce<U, Load>;

#[derive(Clone, Serialize)]
pub enum Part {
    List(Text<List>),
    Unknown,
}

pub fn string<A>(string: &str) -> View<A> {
    View::Base(Item::Base(view::Ace::Bare(string.into())))
}

pub fn leaf<A>(string: &str) -> View<A> {
    View::Base(Item::Base(view::Ace::Link(string.ace())))
}
