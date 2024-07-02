use crate::text::*;
use graph::*;

pub use attribute::*;
pub use element::Element;
pub use tag::*;

#[cfg(test)]
mod tests;

mod attribute;
mod element;
mod tag;

pub struct Html(UnitTasker<Element, Work>);

impl Html {
    fn new(element: Element) -> Self {
        Self(UnitTasker::new(element))
    }
    pub fn text(&self) -> Text<List> {
        if let Load::Text(text) = self.0.solve_task(Task::Text) {
            return text;
        }
        panic!("should have returned text");
    }
}

impl Default for Html {
    fn default() -> Self {
        Self(UnitTasker::new(Element::new()))
    }
}

type Work = graph::Work<Task, Load>;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub enum Task {
    #[default]
    Text,
}

#[derive(Clone)]
pub enum Load {
    Text(Text<List>),
    Serial(String),
}

impl Default for Load {
    fn default() -> Self {
        Load::Text(Text::new(List::new()))
    }
}

enum Item {
    String(String),
    // Text(TextSolver),
    Html(Html),
}

impl Item {
    fn collect(&self, list: &mut List, reactor: &Reactor) {
        match self {
            Item::String(string) => list.add_str(string),
            // Item::Text(solver) => text.stemmer(solver, List::add_solver),
            Item::Html(html) => list.add_text(&html.text().with_reactor(reactor)),
        };
    }
}

enum Attribute {
    String(String),
    Text(TextSolver),
}

impl Attribute {
    fn collect(&self, list: &mut List, reactor: &Reactor) {
        match self {
            Attribute::String(string) => list.add_str(string),
            Attribute::Text(solver) => list.add_solver(solver.with_reactor(reactor)),
        };
    }
}

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
