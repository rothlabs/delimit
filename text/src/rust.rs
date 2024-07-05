use crate::plain::{self, TextList};
use graph::*;

pub use struct_rs::StructRs;

#[cfg(test)]
mod tests;

mod struct_rs;

pub type Role = graph::Role<Load, Exact>;

type Item = plain::View<Exact>;

type Load = plain::Role;

type Rust<U> = UnitSolver<U, Load>;

#[derive(Clone)]
pub enum Exact {
    StructRs(Rust<StructRs>),
}

// pub fn struct_rs(rust: &Rust<StructRs>) -> Role {
//     Role {
//         exact: Exact::StructRs(rust.clone()),
//         solver: rust.solver(),
//     }
// }

// enum Item {
//     String(String),
//     Text(plain::Role),
//     Rust(View),
// }

// impl Item {
//     fn add_to_list(&self, list: &mut plain::List, reactor: &Reactor) {
//         match self {
//             Item::String(string) => {list.items.add_str(string);},
//             Item::Text(view) => {list.items.add_role(view, reactor);},
//             Item::Rust(view) => {list.items.add_role(&view.solver.solve(), reactor);},
//         };
//     }
// }

// #[derive(Clone)]
// pub struct View {
//     pub exact: Exact,
//     pub solver: link::Solver<plain::Role>,
// }

// impl View {
//     fn with_reactor(&self, reactor: &Reactor) -> Self {
//         Self {
//             exact: self.exact.clone(),
//             solver: self.solver.with_reactor(reactor),
//         }
//     }
//     pub fn struct_rs(rust: &Rust<StructRs>) -> Self {
//         View {
//             exact: Exact::StructRs(rust.clone()),
//             solver: rust.solver(),
//         }
//     }
// }

// #[derive(Clone)]
// pub enum Exact {
//     StructRs(Rust<StructRs>),
// }
