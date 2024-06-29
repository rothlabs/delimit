use graph::*;

pub use list::{List, TextList};

#[cfg(test)]
mod tests;

mod list;

pub struct Text<T>(pub Solver<T, Work>);

impl<T> Text<T>
where
    T: Solve<Load = Load, Task = Task>,
    T: React + Write + 'static,
{
    fn new(unit: T) -> Self {
        Self(Solver::new(unit))
    }
    pub fn string(&self) -> String {
        if let Load::String(string) = self.0.solve(Task::String) {
            return string;
        }
        panic!("should return Load::String(string)")
    }
    pub fn leaf(&self) -> Leaf<String> {
        if let Load::Leaf(leaf) = self.0.solve(Task::Leaf) {
            return leaf;
        }
        panic!("should return Load::Leaf(Leaf<String>)")
    }
    pub fn stemmer<S: WithReactor, F: FnOnce(&mut T, S)>(&self, stem: &S, add_stem: F) {
        self.0.stemmer(stem, add_stem);
    }
    pub fn writer<F: FnOnce(&mut T::Unit)>(&self, write: F) {
        self.0.writer(write);
    }
}

pub enum Item {
    String(String),
    Leaf(Leaf<String>),
    // Text(Text),
}

impl Item {
    fn read<F: FnOnce(&String)>(&self, read: F) {
        match self {
            Item::String(string) => read(string),
            Item::Leaf(leaf) => leaf.reader(read),
            // Stem::Text(t) => t.leaf().read(f),
        };
    }
}

type Work = graph::Work<Task, Load>;

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub enum Task {
    #[default]
    String,
    Leaf,
}

#[derive(Clone)] // EnumAsInner
pub enum Load {
    String(String),
    Leaf(Leaf<String>),
}

impl Default for Load {
    fn default() -> Self {
        Load::String(String::new())
    }
}

// impl<T: Write> Writer for Text<T> {
//     type Unit = T::Unit;
//     fn writer<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
//         self.0.writer(write);
//     }
// }

// impl<U> Stemmer for Text<U>
// where
//     U: React + 'static,
// {
//     type Unit = U;
//     fn stemmer<T: WithReactor, F: FnOnce(&mut U, T)>(&self, stem: &T, add_stem: F) {
//         self.0.stemmer(stem, add_stem);
//     }
// }

// impl<T: Read> Reader for Text<T> {
//     type Unit = T::Unit;
//     fn read<F: FnOnce(&Self::Unit)>(&self, read: F) {
//         self.0.read(read);
//     }
// }

// #[derive(Default)]
// struct Work(node::Work<Task, Load>);

// impl<T> Solve for Text<T>
// where
//     T: Solve<Load = Load, Task = Task>,
// {
//     type Load = Load;
//     type Task = Task;
//     fn solve(&self, task: Self::Task) -> Self::Load {
//         self.0.solve(task)
//     }
// }

// impl FromUnit for Text {
//     type Unit = Box<dyn Unit>;
//     fn from_unit(unit: Self::Unit) -> Self {
//         Self(Solver::from_unit(unit))
//     }
// }

// struct Pair(node::Pair<Box<dyn Unit>, Text>);

// impl FromUnit for Pair {
//     type Unit = Box<dyn Unit>;
//     fn from_unit(unit: Self::Unit) -> Self {
//         Self(node::Pair::from_unit(unit))
//     }
// }

// use serde::Serialize;

// use dyn_clone::{clone_trait_object, DynClone};
// use enum_as_inner::EnumAsInner;
// use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

// use graph::{
//     self,
//     link::{Leaf, Read, Solve, Solver},
//     AddStem, FromReactor, FromUnit, React, Reactor,
// };

// pub mod list;
// pub use list::list;

// pub fn text(unit: Box<dyn Unit>) -> Text {
//     Text(Solver::from_unit(unit))
// }

// #[derive(Clone, Serialize)]
// pub struct Text(pub Solver<Box<dyn Unit>, Task, Load, Stem>);

// impl Text {
//     pub fn leaf(&self) -> Leaf<String> {
//         self.0.solve(Task::Leaf).as_leaf().unwrap().to_owned()
//     }
//     pub fn serial(&self) -> String {
//         self.0.solve(Task::Serial).as_string().unwrap().to_owned()
//     }
//     pub fn string(&self) -> String {
//         self.0.solve(Task::String).as_string().unwrap().to_owned()
//     }
//     pub fn add_leaf(&mut self, leaf: &Leaf<String>) {
//         self.0.add_stem(Stem::Leaf(leaf.clone()));
//     }
// }

// impl FromReactor for Text {
//     fn from_reactor(&self, root: Reactor) -> Self {
//         Self(self.0.from_reactor(root))
//     }
// }

// clone_trait_object!(Unit);
// serialize_trait_object!(Unit);
// pub trait Unit: DynClone + DynSerialize {
//     fn leaf(&self) -> Leaf<String>;
//     fn string(&self) -> String;
//     fn serial(&self) -> String;
//     fn add_item(&mut self, stem: Stem);
// }

// impl graph::Solve for Box<dyn Unit> {
//     type Load = Load;
//     type Task = Task;
//     fn solve(&mut self, task: Task) -> Load {
//         match task {
//             Task::Leaf => Load::Leaf(self.leaf()),
//             Task::Serial => Load::String(self.serial()),
//             Task::String => Load::String(self.string()),
//         }
//     }
// }

// impl React for Box<dyn Unit> {
//     fn react(&mut self) {
//         println!("text unit reacted!");
//     }
// }

// impl AddStem for Box<dyn Unit> {
//     type Stem = Stem;
//     fn add_stem(&mut self, stem: Self::Stem) {
//         self.add_item(stem);
//     }
// }

// #[derive(Clone, Eq, PartialEq, Hash)]
// pub enum Task {
//     Leaf,
//     String,
//     Serial,
// }

// #[derive(Clone, EnumAsInner)]
// pub enum Load {
//     Leaf(Leaf<String>),
//     String(String),
// }

// #[derive(Clone, Serialize)]
// pub enum Stem {
//     Leaf(Leaf<String>),
//     Text(Text),
// }

// impl Stem {
//     fn read<F: FnOnce(&String)>(&self, f: F) {
//         match self {
//             Stem::Leaf(l) => l.read(f),
//             Stem::Text(t) => t.leaf().read(f),
//         };
//     }
// }

// impl FromReactor for Stem {
//     fn from_reactor(&self, reactor: Reactor) -> Self {
//         match self {
//             Self::Leaf(leaf) => Self::Leaf(leaf.from_reactor(reactor)),
//             Self::Text(text) => Self::Text(text.from_reactor(reactor))
//         }
//     }
// }
