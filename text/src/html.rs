use crate::plain::{self, *};
use graph::*;

pub use attribute::*;
pub use element::Element;
pub use tag::*;

#[cfg(test)]
mod tests;

mod attribute;
mod element;
mod tag;

pub struct Html(UnitSolver<Element, plain::View>);

impl Html {
    fn new(element: Element) -> Self {
        Self(UnitSolver::new(element))
    }
    pub fn solve(&self) -> plain::View {
        self.0.solve()
    }
}

impl Default for Html {
    fn default() -> Self {
        Self(UnitSolver::new(Element::new()))
    }
}

enum Item {
    String(String),
    Text(plain::View),
    Html(Html),
}

impl Item {
    fn collect(&self, list: &mut List, reactor: &Reactor) {
        match self {
            Item::String(string) => list.add_str(string),
            Item::Text(view) => list.add_view(view, reactor),
            Item::Html(html) => list.add_view(&html.solve(), reactor),
        };
    }
}

enum Attribute {
    String(String),
    Text(plain::View),
}

impl Attribute {
    fn collect(&self, list: &mut List, reactor: &Reactor) {
        match self {
            Attribute::String(string) => list.add_str(string),
            Attribute::Text(view) => list.add_view(view, reactor),
        };
    }
}

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
