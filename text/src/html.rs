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

pub type Role = role::Ploy<Actual, Load>;

type Load = plain::Role;
type Item = plain::View<Actual>;
type Html<U> = Pair<U, Load>;

#[derive(Clone)]
pub enum Actual {
    Element(Html<Element>),
    Tag(Html<Tag>),
    Attribute(Html<Attribute>),
}