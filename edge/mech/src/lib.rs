use graph::*;

pub struct Vector {
    units: Vec<Hub<f64>>,
}

impl Solve for Vector {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut out = vec![];
        for unit in &self.units {
            out.push(unit.base().await?);
        }
        Ok(out.leaf().hub())
    }
}

impl Adapt for Vector {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.units.deal("units", deal)
    }
}

pub struct Matrix {
    vectors: Vec<Hub<Vec<f64>>>,
}

impl Solve for Matrix {
    type Base = Vec<f64>;
    async fn solve(&self) -> Result<Hub<Vec<f64>>> {
        let mut out = vec![];
        for vector in &self.vectors {
            out.extend(vector.base().await?);
        }
        Ok(out.leaf().hub())
    }
}

impl Adapt for Matrix {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.vectors.deal("vectors", deal)
    }
}
