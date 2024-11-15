pub use arithmetic::*;

use derive_builder::*;
use graph::*;
use node_derive::*;
use std::fmt::Debug;

mod arithmetic;

pub fn vector<T>() -> VectorBuilder<T> {
    VectorBuilder::default()
}

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Vector<T> {
    #[builder(setter(each(name = "field", into)))]
    pub fields: Vec<Hub<T>>,
}

impl<T> Solve for Vector<T>
where
    T: 'static + Clone + SendSync + Debug,
{
    type Base = Vec<T>;
    async fn solve(&self) -> node::Result<Hub<Vec<T>>> {
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
    fields: Vec<Hub<T>>,
}

impl<T> Solve for Join<T>
where
    T: 'static + Clone + SendSync + Debug + Default,
{
    type Base = T;
    async fn solve(&self) -> node::Result<Hub<Self::Base>> {
        self.fields.depend().await?;
        Ok(T::default().into())
    }
}

pub fn writer<T>(source: Hub<T>) -> WriterBuilder<T> {
    WriterBuilder::default().source(source)
}

#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
pub struct Writer<T> {
    source: Hub<T>,
    target: Leaf<T>,
}

impl<T> Solve for Writer<T>
where
    T: 'static + Clone + SendSync + Debug,
{
    type Base = ();
    async fn solve(&self) -> node::Result<Hub<()>> {
        let value = self.source.base().await?;
        println!("writing to commands");
        self.target.write(|x| *x = value).await?;
        Ok(().into())
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
