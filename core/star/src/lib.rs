use derive_builder::Builder;
use graph::*;
use node_derive::*;

#[derive(Builder, Adapt)]
#[builder(pattern = "owned")]
pub struct Vector {
    #[builder(setter(each(name = "field", into)))]
    fields: Vec<Hub<f64>>,
}

impl Solve for Vector {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut vector = vec![];
        for field in &self.fields {
            vector.push(field.base().await?);
        }
        Ok(vector.into_leaf().hub())
    }
    fn rank(&self) -> u16 {
        1
    }
}

#[derive(Builder, Adapt)]
#[builder(pattern = "owned")]
pub struct Matrix {
    #[builder(setter(each(name = "vector", into)))]
    vectors: Vec<Hub<Vec<f64>>>,
}

impl Solve for Matrix {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut matrix = vec![];
        for vector in &self.vectors {
            matrix.extend(vector.base().await?);
        }
        Ok(matrix.into_leaf().hub())
    }
    fn rank(&self) -> u16 {
        1
    }
}