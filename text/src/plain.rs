pub use list::{List, ToList};
pub use doc::Doc;

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
mod doc;

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

/// Plain test to use as stem
pub type Stem = graph::view::end::Ploy<Part, String>;

#[derive(Clone, Serialize)]
pub enum Part {
    List(Link<List>),
    Unknown,
}

type Link<U> = Deuce<U>;
type Load = Ace<String>;


// pub fn ace<A>(ace: &Ace<String>) -> view::Ploy<A> {
//     View::Base(Stem::Base(graph::view::End::Link(ace.clone())))
// }

// pub fn str<A>(str: &str) -> view::Ploy<A> {
//     View::Base(Stem::Base(graph::view::End::Bare(str.into())))
// }