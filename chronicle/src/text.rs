use std::clone;

use serde::Serialize;

use dyn_clone::{clone_trait_object, DynClone};
use enum_as_inner::EnumAsInner;
use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use graph::{self, Stem};
use graph::{Edge, LeafStr, Solve};

pub mod unit;
pub use unit::list;

#[derive(Clone, Serialize)]
pub struct Text(pub Edge<Box<dyn Unit>, Task, Goal>);

impl Solve<Task, Goal> for Box<dyn Unit> {
    fn solve(&self, task: Task) -> Option<Goal> {
        match task {
            Task::Leaf(_) => Some(Goal::Leaf(self.leaf())),
            Task::Serial(_) => Some(Goal::String(self.serial())),
            Task::String(_) => Some(Goal::String(self.string())),
        }
    }
    fn stems(&self) -> Vec<Box<dyn Stem>> {
        self.all_stems()
    }
}

impl Text {
    pub fn leaf(&self) -> LeafStr {
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

// impl Stem for Text {
    
// }

clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + DynSerialize {
    fn leaf(&self) -> LeafStr;
    fn string(&self) -> String;
    fn serial(&self) -> String;
    fn all_stems(&self) -> Vec<Box<dyn Stem>>;
}

pub fn text(unit: impl Unit + 'static) -> Text {
    Text(Edge::new(Box::new(unit)))
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Task {
    Leaf(()),
    String(()),
    Serial(()),
}

//impl graph::Task for Task {}

#[derive(Clone, EnumAsInner)]
pub enum Goal {
    Leaf(LeafStr),
    String(String),
}
