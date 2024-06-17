use dyn_clone::DynClone;
use enum_as_inner::EnumAsInner;
use erased_serde::serialize_trait_object;
use graph::{Edge, Snap, Solve, LeafStr};
use serde::Serialize;

pub mod unit;


pub enum Task {
    Leaf(()),
    Serial(()),
} 

#[derive(EnumAsInner)]
pub enum Goal {
    Leaf(LeafStr),
    Serial(String),
    // String(String),
} 

pub fn text(snap: &Snap, unit: impl Unit + 'static) -> Text {
    Text(snap.edge(Box::new(unit)))
}

impl Solve<Task, Goal> for Box<dyn Unit> {
    fn solve(&self, task: Task) -> Option<Goal> {
        match task {
            Task::Leaf(_) => Some(Goal::Leaf(self.leaf())),
            Task::Serial(_) => Some(Goal::Serial(self.serial())),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct Text(pub Edge<Box<dyn Unit>, Task, Goal>);

impl Text {
    pub fn leaf(&self) -> LeafStr { 
        self.0.solve(Task::Leaf(())).unwrap().as_leaf().unwrap().to_owned()
    }
    pub fn serial(&self) -> String {
        self.0.solve(Task::Serial(())).unwrap().as_serial().unwrap().to_owned()
    }
    // pub fn string(&self) -> String {
    //     self.leaf().read().read().clone()
    // }
}

dyn_clone::clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + erased_serde::Serialize {
    fn leaf(&self) -> LeafStr;
    fn serial(&self) -> String;
}


        // if let Goal::Leaf(goal) = self.0.solve(Task::Leaf(())).unwrap() {
        //     goal
        // } else {
        //     panic!("wrong variant")
        // }
        // /wow.<Goal as Into<T>>::into().unwrap()
        //self.0.read(&|unit| unit.leaf())