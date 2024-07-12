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

pub type Role = graph::SolveRole<Load, Exact>;

type Load = plain::Role;
type Item = plain::View<Exact>;
type Html<U> = Pair<U, Load>;

#[derive(Clone)]
pub enum Exact {
    Element(Html<Element>),
    Tag(Html<Tag>),
    Attribute(Html<Attribute>),
}

// pub struct Html(UnitSolver<Element, plain::Role>);

// impl Html {
//     fn new(element: Element) -> Self {
//         Self(UnitSolver::new(element))
//     }
//     pub fn solve(&self) -> plain::Role {
//         self.0.solve()
//     }
// }

// impl Default for Html {
//     fn default() -> Self {
//         Self(UnitSolver::new(Element::new()))
//     }
// }

// enum Item {
//     String(String),
//     Text(plain::Role),
//     Html(Html),
// }

// impl Item {
//     fn collect(&self, pack: &mut Pack<List>) {
//         match self {
//             Item::String(string) => {
//                 pack.unit.items.add_bare(string);
//             }
//             Item::Text(view) => {
//                 pack.unit.items.add_role(view, pack.reactor);
//             }
//             Item::Html(html) => {
//                 pack.unit.items.add_role(&html.solve(), pack.reactor);
//             }
//         };
//     }
// }

// enum Attribute {
//     String(String),
//     Text(plain::Role),
// }

// impl Attribute {
//     fn collect(&self, pack: &mut Pack<List>) {
//         match self {
//             Attribute::String(string) => {
//                 pack.unit.items.add_str(string);
//             }
//             Attribute::Text(view) => {
//                 pack.unit.items.add_role(view, pack.reactor);
//             }
//         };
//     }
// }

// pub fn text(&self) -> Text<List> {
//     if let Load::Text(text) = self.0.solve_task(Task::Text) {
//         return text;
//     }
//     panic!("should have returned text");
// }

// impl Item {
//     fn collect(&self, text: &Text<List>) {
//         match self {
//             Item::String(string) => text.writer(|list| list.add_str(string)),
//             // Item::Text(solver) => text.stemmer(solver, List::add_solver),
//             Item::Html(html) => text.stemmer(&html.text(), List::add_text), // list.add_text(&h.text()),
//         };
//     }
// }

// impl Attribute {
//     fn collect(&self, text: &Text<List>) {
//         match self {
//             Attribute::String(string) => text.writer(|list| list.add_str(string)),
//             Attribute::Text(solver) => text.stemmer(solver, List::add_solver),
//         };
//     }
// }
