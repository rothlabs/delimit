pub use doc::*;

use super::*;
use attribute::*;
use element::*;
use graph::*;
use tag::*;

mod attribute;
mod doc;
mod element;
mod tag;
#[cfg(test)]
mod tests;

/// HTML Pipe
pub type Pipe = graph::Pipe<Role>;

/// HTML Role
pub type Role = role::Ploy<Part, Load>;

#[derive(Clone)]
pub enum Part {
    Element(Link<Element>),
    Tag(Link<Tag>),
    Attribute(Link<Attribute>),
}

/// HTML link
type Link<U> = Deuce<U>;

// HTML load
type Load = plain::Role;

/// HTML stem view
type Stem = plain::view::Ploy<Part>;
