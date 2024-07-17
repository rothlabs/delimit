use crate::plain::{self, *};
use graph::*;

pub use attribute::*;
pub use doc::*;
pub use element::Element;
pub use tag::*;

#[cfg(test)]
mod tests;

mod attribute;
mod doc;
mod element;
mod tag;

/// HTML to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

type Load = plain::Role;
type Stem = plain::view::Ploy<Part>;
type Link<U> = Deuce<U, Load>;

#[derive(Clone)]
pub enum Part {
    Element(Link<Element>),
    Tag(Link<Tag>),
    Attribute(Link<Attribute>),
}
