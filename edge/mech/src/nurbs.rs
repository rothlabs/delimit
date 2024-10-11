// use super::*;

// pub struct Nurbs {
//     gpu: Gpu,
// }

// impl Act for Nurbs {
//     async fn act(&self) -> graph::Result<()> {
//         let mut encoder = self.gpu.encoder();
//         encoder
//             .compute()
//             .pipe(&pipe)
//             .bind(0, &bind, &[])
//             .dispatch(count, 1, 1);
//         encoder.submit();
//         Ok(())
//     }
// }