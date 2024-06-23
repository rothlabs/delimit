use serde::Serialize;

use dyn_clone::{clone_trait_object, DynClone};
use enum_as_inner::EnumAsInner;
use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use graph::{
    New, 
    node, 
    link::{Leaf, Solve, Solver},
};

pub mod unit;
pub use unit::list;

#[derive(Clone, Serialize)]
pub struct Text(pub Solver<Box<dyn Unit>, Task, Load>);
// pub struct Text(pub Link<Edge<Reactor, Solver<Box<dyn Unit>, Task, Load>>>);

impl node::Solve for Box<dyn Unit> {
    type Load = Load;
    type Task = Task;
    fn solve(&mut self, task: Task) -> Load {
        match task {
            Task::Leaf => Load::Leaf(self.leaf()),
            Task::Serial => Load::String(self.serial()),
            Task::String => Load::String(self.string()),
        }
    }
}

impl Text {
    pub fn leaf(&self) -> Leaf<String> {
        self.0.solve(Task::Leaf).as_leaf().unwrap().to_owned()
    }
    pub fn serial(&self) -> String {
        self.0.solve(Task::Serial).as_string().unwrap().to_owned()
    }
    pub fn string(&self) -> String {
        self.0.solve(Task::String).as_string().unwrap().to_owned()
    }
}

clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + DynSerialize {
    fn leaf(&self) -> Leaf<String>;
    fn string(&self) -> String;
    fn serial(&self) -> String;
}

pub fn text(unit: Box<dyn Unit>) -> Text {
    Text(Solver::new(unit))
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Task {
    Leaf,
    String,
    Serial,
}

#[derive(Clone, EnumAsInner)]
pub enum Load {
    Leaf(Leaf<String>),
    String(String),
}
