use serde::Serialize;

use dyn_clone::{clone_trait_object, DynClone};
use enum_as_inner::EnumAsInner;
use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use graph::{self, Stem};
use graph::{LeafStr, Solve};

pub mod unit;
pub use unit::list;

#[derive(Clone, Serialize)]
pub struct Text(pub Stem<Box<dyn Unit>, Task, Load>);

impl Solve<Task, Load> for Box<dyn Unit> {
    fn solve(&self, task: Task) -> Option<Load> {
        match task {
            Task::Leaf(_) => Some(Load::Leaf(self.leaf())),
            Task::Serial(_) => Some(Load::String(self.serial())),
            Task::String(_) => Some(Load::String(self.string())),
        }
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

clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + DynSerialize {
    fn leaf(&self) -> LeafStr;
    fn string(&self) -> String;
    fn serial(&self) -> String;
}

pub fn text(unit: impl Unit + 'static) -> Text {
    Text(Stem::new(Box::new(unit)))
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Task {
    Leaf(()),
    String(()),
    Serial(()),
}

#[derive(Clone, EnumAsInner)]
pub enum Load {
    Leaf(LeafStr),
    String(String),
}
