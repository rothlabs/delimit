pub use calc::*;
pub use transfer::*;

use derive_builder::*;
use graph::*;
use node_derive::*;
use std::fmt::Debug;

mod calc;
mod transfer;

pub fn vector<T>() -> VectorBuilder<T> {
    VectorBuilder::default()
}

#[derive(Builder, BuildGate, Back, Debug)]
#[builder(pattern = "owned")]
pub struct Vector<T> {
    #[builder(setter(each(name = "field", into)))]
    pub fields: Vec<Hub<T>>,
}

impl<T> Solve for Vector<T>
where
    T: 'static + Gather,
{
    type Base = Vec<T>;
    async fn solve(&self) -> node::Result<Vec<T>> {
        let mut vector = vec![];
        for field in &self.fields {
            vector.push(field.base().await?);
        }
        Ok(vector.into_leaf().hub())
    }
}

#[derive(Debug, Back, Builder, BuildGate)]
#[builder(build_fn(error = "graph::Error"))]
pub struct Join<T> {
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> Solve for Join<T>
where
    T: 'static + Gather + Default,
{
    type Base = T;
    async fn solve(&self) -> node::Result<Self::Base> {
        self.fields.depend().await?;
        Ok(T::default().into())
    }
}