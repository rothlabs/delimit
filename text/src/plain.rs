use serde::Serialize;

use graph::*;

pub use list::{List, ToList};

#[cfg(test)]
mod tests;

mod list;
pub mod view {
    use super::*;
    /// Plain text Stem to use inside units of super graphs
    pub type Ploy<P> = graph::View<role::Ploy<P, Role>, Stem>;
}

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

type Link<U> = Deuce<U, Load>;
type Stem = graph::view::ace::Ploy<Part, String>;
type Load = Ace<String>;

#[derive(Clone, Serialize)]
pub enum Part {
    List(Link<List>),
    Unknown,
}

pub fn str<A>(str: &str) -> view::Ploy<A> {
    View::Base(Stem::Base(graph::view::Ace::Bare(str.into())))
}

pub fn ace<A>(str: &str) -> view::Ploy<A> {
    View::Base(Stem::Base(graph::view::Ace::Link(str.ace())))
}
