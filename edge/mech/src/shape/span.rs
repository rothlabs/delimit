use super::*;

#[derive(Clone, Debug)]
pub struct Vector {
    pub rule: VectorRule,
    pub hedge: Hedge,
}

#[derive(Clone, Debug)]
enum VectorRule {
    Direct,
    Nurbs,
}

#[derive(Clone, Debug)]
pub struct Matrix {
    rule: MatrixRule,
    hedge: Hedge,
}

#[derive(Clone, Debug)]
enum MatrixRule {
    Direct,
    Extrude,
    Revolve,
}