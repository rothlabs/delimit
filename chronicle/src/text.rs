use dyn_clone::DynClone;
use enum_as_inner::EnumAsInner;
use erased_serde::serialize_trait_object;
use graph::{Edge, LeafStr, Snap, Solve};
use serde::Serialize;

pub mod unit;

#[derive(Clone, Serialize)]
pub struct Text(pub Edge<Box<dyn Unit>, Task, Goal>);

impl Solve<Task, Goal> for Box<dyn Unit> {
    fn solve(&self, task: Task) -> Option<Goal> {
        match task.query {
            Query::Leaf(_) => Some(Goal::Leaf(self.leaf())),
            Query::Serial(_) => Some(Goal::String(self.serial())),
            Query::String(_) => Some(Goal::String(self.string())),
        }
    }
}

impl Text {
    pub fn leaf(&self) -> LeafStr { 
        let task = Task {query: Query::Leaf(())};
        self.0.solve(task).as_leaf().unwrap().to_owned()
    }
    pub fn serial(&self) -> String {
        let task = Task {query: Query::Serial(())};
        self.0.solve(task).as_string().unwrap().to_owned()
    }
    pub fn string(&self) -> String {
        let task = Task {query: Query::String(())};
        self.0.solve(task).as_string().unwrap().to_owned()
    }
}

dyn_clone::clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + erased_serde::Serialize {
    fn leaf(&self) -> LeafStr;
    fn string(&self) -> String;
    fn serial(&self) -> String;
}

pub fn text(unit: impl Unit + 'static) -> Text {
    Text(Edge::new(Box::new(unit)))
}

pub struct Task {
    //pub snap: Snap,
    pub query: Query,
}

pub enum Query {
    Leaf(()),
    String(()),
    Serial(()),
} 

#[derive(Clone, EnumAsInner)]
pub enum Goal {
    Leaf(LeafStr),
    String(String),
} 


        // if let Goal::Leaf(goal) = self.0.solve(Task::Leaf(())).unwrap() {
        //     goal
        // } else {
        //     panic!("wrong variant")
        // }
        // /wow.<Goal as Into<T>>::into().unwrap()
        //self.0.read(&|unit| unit.leaf())