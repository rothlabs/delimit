use super::*;

#[derive(Builder, Clone, Debug)]
pub struct Grid {
    shape: Hub<Shape>
}

// impl Solve for Grid {
//     type Base = ;
// }