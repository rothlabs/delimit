use super::*;

#[derive(Clone, Debug)]
pub struct Vector {
    pub nurbs: Option<Hedge>,
}

#[derive(Clone, Debug)]
pub struct Matrix {
    pub extrude: Option<Hedge>,
    pub revolve: Option<Hedge>,
}

// #[derive(Clone, Debug)]
// pub enum VectorRule {
//     Direct,
//     Nurbs,
// }

// #[derive(Clone, Debug)]
// enum MatrixRule {
//     Direct,
//     Extrude,
//     Revolve,
// }
