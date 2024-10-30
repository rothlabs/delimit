use derive_builder::*;
use graph::*;
use node_derive::*;
use std::{fmt::Debug, ops};

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

// TODO: make trait to make new Sum with other
#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Sum<T> {
    #[builder(setter(each(name = "field", into)))]
    pub fields: Vec<Hub<T>>,
}

impl<T> Solve for Sum<T>
where
    T: 'static + Clone + SendSync + Debug + Default + ops::AddAssign<T>,
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let mut sum = T::default();
        for field in &self.fields {
            sum += field.base().await?;
        }
        Ok(sum.into_leaf().into())
    }
}

// TODO: make trait on Hub<T: Number> to make new Divide
#[derive(Builder, Back, Gate, Debug)]
#[builder(pattern = "owned")]
pub struct Divide<T> {
    pub dividend: Hub<T>,
    pub divisor: Hub<T>,
}

impl<T> Solve for Divide<T>
where
    T: 'static + Clone + SendSync + Debug + Default + ops::Div<Output = T>,
{
    type Base = T;
    async fn solve(&self) -> graph::Result<Hub<T>> {
        let quotient = self.dividend.base().await? / self.divisor.base().await?;
        Ok(quotient.into_leaf().into())
    }
}



// impl<T> Adapt for Vector<T>
// where
//     T: 'static + Clone + SendSync + Debug,
// {
//     fn back(&mut self, back: &Back) -> graph::Result<()> {
//         self.fields.back(back)
//     }
// }

// #[derive(Builder, Adapt)]
// #[builder(pattern = "owned")]
// pub struct Matrix {
//     #[builder(setter(each(name = "vector", into)))]
//     vectors: Vec<Hub<Vec<f64>>>,
// }

// impl Solve for Matrix {
//     type Base = Vec<f64>;
//     async fn solve(&self) -> Result<Hub<Vec<f64>>> {
//         let mut matrix = vec![];
//         for vector in &self.vectors {
//             matrix.extend(vector.base().await?);
//         }
//         Ok(matrix.into_leaf().hub())
//     }
//     fn rank(&self) -> u16 {
//         1
//     }
// }

// #[derive(Builder, Adapt, Gate, Debug)]
// #[builder(pattern = "owned")]
// pub struct Vector<T> {
//     #[builder(setter(each(name = "field", into)))]
//     fields: Vec<Hub<T>>,
// }

// impl<T> Solve for Vector<T> {
//     type Base = Vec<f64>;
//     async fn solve(&self) -> Result<Hub<Vec<f64>>> {
//         let mut vector = vec![];
//         for field in &self.fields {
//             vector.push(field.base().await?);
//         }
//         Ok(vector.into_leaf().hub())
//     }
//     fn rank(&self) -> u16 {
//         1
//     }
// }
