pub use list::{List, ToList};

use graph::*;
use serde::Serialize;

pub mod view {
    use super::*;
    /// Plain text Link to use inside units of super graphs
    pub type Ploy<P> = View<role::Ploy<P, Role>, Stem>;
}

mod list;
#[cfg(test)]
mod tests;

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

#[derive(Clone, Serialize)]
pub enum Part {
    List(Link<List>),
    Unknown,
}

pub fn str<A>(str: &str) -> view::Ploy<A> {
    View::Base(Stem::Base(graph::view::End::Bare(str.into())))
}

pub fn ace<A>(str: &str) -> view::Ploy<A> {
    View::Base(Stem::Base(graph::view::End::Link(str.ace())))
}

type Link<U> = Deuce<U, Load>;
type Stem = graph::view::end::Ploy<Part, String>;
type Load = Ace<String>;
