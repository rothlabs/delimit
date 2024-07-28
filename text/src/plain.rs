pub use doc::Doc;
pub use list::List; //{List, ToList};

// use super::*;
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

// #[derive(Serialize)]
pub enum Part {
    List(Deuce<List>),
}

/// Plain text to use as Load of super graphs
pub type Role = role::Ploy<OldPart, Load>;

/// Plain text to use as graph stem
pub type Stem = graph::view::Ploy<OldPart, String>;

#[derive(Clone, Serialize)]
pub enum OldPart {
    List(Link<List>),
}

type Link<U> = Deuce<U>;
type Load = Ace<String>;