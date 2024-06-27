use graph::*;

pub mod list;

pub struct Text<T>(Solver<T, Work>);

impl<T> Text<T> { 
    fn from_unit(unit: T) -> Self {
        Self(Solver::from_unit(unit))
    }
}

impl<T: Write> Writer for Text<T> {
    type Unit = T::Unit;
    fn write<F: FnOnce(&mut Self::Unit)>(&self, write: F) {
        self.0.write(write);
    }
}

impl<T> ToString for Text<T> 
where 
    T: Solve<Load = Load, Task = Task>,
{
    fn string(&self) -> String {
        if let Load::String(string) = self.0.solve(Task::String) {
            string
        } else {
            panic!("task produced wrong load")
        }
    }
}

pub enum Stem {
    String(String),
    Leaf(Leaf<String>),
    // Text(Text),
}

impl Stem {
    fn read<F: FnOnce(&String)>(&self, f: F) {
        match self {
            Stem::String(s) => f(s),
            Stem::Leaf(l) => l.read(f),
            //Stem::Text(t) => t.leaf().read(f),
        };
    }
}

type Work = node::Work<Task, Load>;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Task {
    String,
    // Leaf,
    // Serial,
}

impl Default for Task {
    fn default() -> Self {
        Task::String
    }
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
