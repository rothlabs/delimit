pub use facet::Facets;

// use mecha::*;
use graph::*;

mod facet;

pub type Role = role::Ploy<Part, ()>;

type Link<U> = Deuce<U, ()>;

#[derive(Clone)]
pub enum Part {
    Facets(Link<Facets>),
}






