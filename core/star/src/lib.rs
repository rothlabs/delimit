pub use arithmetic::*;

use derive_builder::*;
use graph::*;
use node_derive::*;
use std::fmt::Debug;

mod arithmetic;

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
    T: 'static + Clone + SendSync + Debug,
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
    T: 'static + Clone + SendSync + Debug + Default,
{
    type Base = T;
    async fn solve(&self) -> node::Result<Self::Base> {
        self.fields.depend().await?;
        Ok(T::default().into())
    }
}

pub fn transfer<T>(source: Hub<T>, target: impl Into<Leaf<T>>) -> Result<Hub<()>>
where
    T: 'static + Gather,
{
    Transfer {
        source,
        target: target.into(),
    }
    .hub()
}

#[derive(Gate, Back, Debug)]
pub struct Transfer<T> {
    pub source: Hub<T>,
    pub target: Leaf<T>,
}

impl<T> Act for Transfer<T>
where
    T: 'static + Clone + SendSync + Debug,
{
    async fn act(&self) -> node::Action {
        let value = self.source.base().await?;
        self.target.write(|x| *x = value).await?;
        acted()
    }
}

// pub fn writer<T>(source: Hub<T>) -> WriterBuilder<T> {
//     WriterBuilder::default().source(source)
// }

// #[derive(Builder, Back, Gate, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct Writer<T> {
//     source: Hub<T>,
//     target: Leaf<T>,
// }

// impl<T> Act for Writer<T>
// where
//     T: 'static + Clone + SendSync + Debug,
// {
//     async fn act(&self) -> node::Action {
//         let value = self.source.base().await?;
//         self.target.write(|x| *x = value).await?;
//         acted()
//     }
// }
