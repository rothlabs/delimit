use serde::Serialize;

use dyn_clone::{clone_trait_object, DynClone};
use enum_as_inner::EnumAsInner;
use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use graph::{self, link::{New, Solver, Solve}, node::{self, Leaf}};

pub mod unit;
pub use unit::list;


#[derive(Clone, Serialize)]
pub struct Text(
    pub Solver<Box<dyn Unit>, Task, Load>
); 
// pub struct Text(pub Link<Edge<Reactor, Solver<Box<dyn Unit>, Task, Load>>>); 

impl node::Solve for Box<dyn Unit> {
    type Load = Load;
    type Task = Task;
    fn solve(&mut self, task: Task) -> Load {
        match task {
            Task::Leaf(_) => Load::Leaf(self.leaf()),
            Task::Serial(_) => Load::String(self.serial()),
            Task::String(_) => Load::String(self.string()),
        }
    }
}

impl Text {
    pub fn leaf(&self) -> Leaf<String> {
        let task = Task::Leaf(());
        self.0.solve(task).as_leaf().unwrap().to_owned()
    }
    pub fn serial(&self) -> String {
        let task = Task::Serial(());
        self.0.solve(task).as_string().unwrap().to_owned()
    }
    pub fn string(&self) -> String {
        let task = Task::String(());
        self.0.solve(task).as_string().unwrap().to_owned()
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
    Leaf(()),
    String(()),
    Serial(()),
}

#[derive(Clone, EnumAsInner)]
pub enum Load {
    Leaf(Leaf<String>),
    String(String),
}
