pub use doc::*;

use graph::*;

use super::*;
use attribute::*;
use element::*;
use tag::*;

#[cfg(test)]
mod tests;

mod attribute;
mod doc;
mod element;
mod tag;

/// HTML to use as Load of super graphs
pub type Role = role::Ploy<Part, Load>;

#[derive(Clone)]
pub enum Part {
    Element(Link<Element>),
    Tag(Link<Tag>),
    Attribute(Link<Attribute>),
}

type Load = plain::Role;
type Stem = plain::view::Ploy<Part>;
type Link<U> = Deuce<U, Load>;
