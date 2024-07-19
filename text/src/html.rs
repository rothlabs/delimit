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

/// HTML role
pub type Role = role::Ploy<Part, Load>;

#[derive(Clone)]
pub enum Part {
    Element(Link<Element>),
    Tag(Link<Tag>),
    Attribute(Link<Attribute>),
}

/// HTML link
type Link<U> = Deuce<U, Load>;

// HTML load
type Load = plain::Role;

/// HTML stem view
type Stem = plain::view::Ploy<Part>;

