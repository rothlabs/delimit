use derive_node::Adapt;
use graph::*;

const BASE: u16 = 1;

#[derive(Adapt)]
pub struct Vector {
    units: Vec<Hub<f64>>,
}

impl Solve for Vector {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut vector = vec![];
        for unit in &self.units {
            vector.push(unit.base().await?);
        }
        Ok(vector.leaf().hub())
    }
    fn rank(&self) -> u16 {
        BASE
    }
}

#[derive(Adapt)]
pub struct Matrix {
    vectors: Vec<Hub<Vec<f64>>>,
}

impl Solve for Matrix {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut matrix = vec![];
        for vector in &self.vectors {
            matrix.extend(vector.base().await?);
        }
        Ok(matrix.leaf().hub())
    }
    fn rank(&self) -> u16 {
        BASE
    }
}
