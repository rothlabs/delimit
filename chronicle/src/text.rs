use dyn_clone::DynClone;
use erased_serde::serialize_trait_object;
use graph::{Compute, Edge};
use serde::Serialize;

pub mod unit;

pub fn text(unit: impl Unit + 'static) -> Text {
    Text(Edge::new(Box::new(unit)))
}

impl<P> Compute<P> for Box<dyn Unit> {
    fn compute(&self) -> Option<P> {
        None
    }
}

#[derive(Clone, Serialize)]
pub struct Text(pub Edge<Box<dyn Unit>, Edge<String, ()>>);

impl Text {
    pub fn leaf(&self) -> Edge<String, ()> {
        self.0.read().read().leaf()
        //self.0.compute::<Edge<String>>();
    }
    pub fn json(&self) -> String {
        self.0.read().read().json()
    }
    pub fn string(&self) -> String {
        self.leaf().read().read().clone()
    }
}

dyn_clone::clone_trait_object!(Unit);
serialize_trait_object!(Unit);
pub trait Unit: DynClone + erased_serde::Serialize {
    // Base<Edge<String,()>> +
    fn leaf(&self) -> Edge<String, ()>;
    fn json(&self) -> String;
}
