pub use doc::Doc;
pub use list::{List, ToList};

use graph::*;
use serde::Serialize;

pub mod view {
    use super::*;
    /// Plain text Link to use inside units of super graphs
    pub type Ploy<P> = View<role::Ploy<P, Role>, Stem>;
}

mod doc;
mod list;
#[cfg(test)]
mod tests;

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

/// Plain text to use as graph stem
pub type Stem = graph::view::end::Ploy<Part, String>;

#[derive(Clone, Serialize)]
pub enum Part {
    List(Link<List>),
    Unknown,
}

type Link<U> = Deuce<U>;
type Load = Ace<String>;
