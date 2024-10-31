pub use arithmetic::*;

use derive_builder::*;
use graph::*;
use node_derive::*;
use std::fmt::Debug;

mod arithmetic;

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Vector<T> {
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>,
}

impl<T> Solve for Vector<T>
where
    T: 'static + Clone + SendSync + Debug,
{
    type Base = Vec<T>;
    async fn solve(&self) -> graph::Result<Hub<Vec<T>>> {
        let mut vector = vec![];
        for field in &self.fields {
            vector.push(field.base().await?);
        }
        Ok(vector.into_leaf().hub())
    }
}

#[derive(Builder, Back, Gate, Debug)]
#[builder(build_fn(error = "graph::Error"))]
pub struct Join<T> {
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<T>>
}

impl<T> Solve for Join<T> 
where 
    T: 'static + Clone + SendSync + Debug
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<Self::Base>> {
        self.fields.depend().await?;
        solve_ok()
    }
}

// pub trait JoinNodes<T> {
//     fn join(&self) -> JoinBuilder<T>;
// }

// impl<T: Clone> JoinNodes<T> for Hub<T> {
//     fn join(&self) -> JoinBuilder<T> {
//         JoinBuilder::default().field(self.clone())
//     }
// }

