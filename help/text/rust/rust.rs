use crate::plain;
use graph::*;

pub use generics::{generics, Generics};
pub use struct_rs::{struct_rs, StructRs};

#[cfg(test)]
mod tests;

mod generics;
mod struct_rs;

pub type Role = graph::Role<Load, Exact>;

type Item = plain::View<Exact>;

type Load = plain::Role;

type Rust<U> = UnitSolver<U, Load>;

#[derive(Clone)]
pub enum Exact {
    StructRs(Rust<StructRs>),
    Generics(Rust<Generics>),
}