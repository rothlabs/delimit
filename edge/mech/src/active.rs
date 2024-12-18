// use super::*;
// use std::fmt::Debug;

// #[derive(Debug, Gate, Back)]
// pub struct Extrude<T> {
//     pub block: Hub<T>,
// }

// impl<T> Solve for Extrude<T>
// where
//     T: 'static + graph::Gather,
// {
//     type Base = Grc<stable::Extrude<T>>;
//     async fn solve(&self) -> node::Result<Self::Base> {
//         let extrude = stable::Extrude {
//             block: self.block.base().await?,
//         };
//         Ok(Grc::new(extrude).into())
//     }
// }
