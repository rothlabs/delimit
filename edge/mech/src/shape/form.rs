use super::*;

#[derive(Clone, Default, Debug)]
pub struct Add {
    pub extrude: Option<Hedge>,
}

// #[derive(Clone, Debug)]
// pub struct Matrix {
//     pub extrude: Option<Hedge>,
//     pub revolve: Option<Hedge>,
// }

#[derive(Clone, Default, Debug)]
pub struct Right {
    // alt name: knots (because hedge only contains knots)
    pub basis: Option<Hedge>,
    pub nurbs: Option<Hedge>,
    // translation
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
