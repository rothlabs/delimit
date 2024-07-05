use graph::*;

pub use gate::{Gate, TextGate};
pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod gate;
mod list;

pub enum View<E> {
    Text(Item),
    Role(graph::Role<Role, E>),
}

impl<E> View<E> {
    pub fn item(&self, reactor: &Reactor) -> Item {
        match self {
            View::Text(item) => item.clone(),
            View::Role(role) => Item::Role(role.solver.solve().with_reactor(reactor)),   
        }
    }
}

pub type Role = graph::Role<Load, Exact>;

type Item = LeafView<String, Exact>;

type Load = Leaf<String>;

type Text<U> = UnitSolver<U, Load>;

#[derive(Clone)]
pub enum Exact {
    List(Text<List>),
    Gate(Text<Gate>),
    Unknown,
}

// pub trait ToExact {
//     fn gate(&self) -> &Text<Gate>;
//     fn list(&self) -> &Text<List>;
// }

// impl ToExact for Role {
//     fn gate(&self) -> &Text<Gate> {
//         if let Exact::Gate(text) = &self.exact {
//             return text
//         }
//         panic!("not a gate")
//     }
//     fn list(&self) -> &Text<List> {
//         if let Exact::List(text) = &self.exact {
//             return text
//         }
//         panic!("not a list")
//     }
// }


// pub fn list(text: &Text<List>) -> Role {
//     Role {
//         exact: Exact::List(text.clone()),
//         solver: text.solver(),
//     }
// }

// pub fn gate(text: &Text<Gate>) -> Role {
//     Role {
//         exact: Exact::Gate(text.clone()),
//         solver: text.solver(),
//     }
// }



// impl Text<Gate> {
//     fn role(&self) -> Role {
//         Role {
//             exact: Exact::Gate(self.clone()),
//             solver: self.solver(),
//         }
//     }
// }

// //pub type Gate = graph::Gate<Item>;

// // make generic?!?! can make generic.
// #[derive(Clone)]
// pub struct View {
//     pub exact: Exact,
//     pub solver: link::Solver<Leaf<String>>,
// }

// impl View {
//     fn with_reactor(&self, reactor: &Reactor) -> Self {
//         Self {
//             exact: self.exact.clone(),
//             solver: self.solver.with_reactor(reactor),
//         }
//     }
//     pub fn list(text: &Text<List>) -> Self {
//         View {
//             exact: Exact::List(text.clone()),
//             solver: text.solver(),
//         }
//     }
// }

// // Viewer?!?! Can make generic and impl solve and read. can access the view.exect to travers
// pub enum Item {
//     String(String),
//     Leaf(Leaf<String>),
//     View(View),
// }

// impl Item {
//     fn read<F: FnOnce(&String)>(&self, read: F) {
//         match self {
//             Item::String(string) => read(string),
//             Item::Leaf(leaf) => leaf.reader(read),
//             Item::View(view) => view.solver.solve().reader(read),
//         };
//     }
// }

// //pub type Gate = graph::Gate<Item>;

// // make generic?!?! can make generic.
// #[derive(Clone)]
// pub struct View {
//     pub exact: Exact,
//     pub solver: link::Solver<Leaf<String>>,
// }

// impl View {
//     fn with_reactor(&self, reactor: &Reactor) -> Self {
//         Self {
//             exact: self.exact.clone(),
//             solver: self.solver.with_reactor(reactor),
//         }
//     }
//     pub fn list(text: &Text<List>) -> Self {
//         View {
//             exact: Exact::List(text.clone()),
//             solver: text.solver(),
//         }
//     }
// }

// #[derive(Clone)]
// pub enum Exact {
//     List(Text<List>),
// }
