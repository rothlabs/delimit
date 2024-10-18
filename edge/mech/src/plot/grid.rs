use super::*;

#[derive(Builder, Clone, Debug)]
pub struct Grid {
    shape: Hub<Shape>,
    count: Hub<u64>,
}

// impl Solve for Grid {
//     type Base = Hedge;
//     async fn solve(&self) -> graph::Result<Hub<Hedge>> {
        
//     }
// }