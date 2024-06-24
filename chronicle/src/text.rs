use serde::Serialize;

use dyn_clone::{clone_trait_object, DynClone};
use enum_as_inner::EnumAsInner;
use erased_serde::{serialize_trait_object, Serialize as DynSerialize};

use graph::{
    self,
    link::{Leaf, Read, Solve, Solver},
    AddLink, FromReactor, FromUnit, React, Reactor,
};

pub mod unit;
pub use unit::list;

pub fn text(unit: Box<dyn Unit>) -> Text {
    Text(Solver::new(unit))
}

#[derive(Clone, Serialize)]
pub struct Text(pub Solver<Box<dyn Unit>, Task, Load, Stem>);

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
    pub fn add_leaf(&mut self, leaf: &Leaf<String>) {
        self.0.add_link(Stem::Leaf(leaf.clone()));
    }
}

impl FromReactor for Text {
    fn from_reactor(&self, root: Reactor) -> Self {
        Self(self.0.from_reactor(root))
    }
}

clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + DynSerialize {
    fn leaf(&self) -> Leaf<String>;
    fn string(&self) -> String;
    fn serial(&self) -> String;
    fn add_item(&mut self, link: Stem);
}

impl graph::Solve for Box<dyn Unit> {
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

impl React for Box<dyn Unit> {
    fn react(&mut self) {
        println!("text unit reacted!");
    }
}

impl AddLink for Box<dyn Unit> {
    type Link = Stem;
    fn add_link(&mut self, link: Self::Link) {
        self.add_item(link);
    }
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

#[derive(Clone, Serialize)]
pub enum Stem {
    Leaf(Leaf<String>),
    Text(Text),
}

impl Stem {
    fn read<F: FnOnce(&String)>(&self, f: F) {
        match self {
            Stem::Leaf(l) => l.read(f),
            Stem::Text(t) => t.leaf().read(f),
        };
    }
}

impl FromReactor for Stem {
    fn from_reactor(&self, reactor: Reactor) -> Self {
        match self {
            Self::Leaf(leaf) => Self::Leaf(leaf.from_reactor(reactor)),
            Self::Text(text) => Self::Text(text.from_reactor(reactor))
        }
    }
}