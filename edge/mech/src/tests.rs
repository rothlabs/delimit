use super::*;

#[tokio::test]
async fn extrude() -> Result<()> {
    let vector = stable::Vector::new(0., 0., []);

    Ok(())
}

// #[tokio::test]
// async fn extrude() -> Result<()> {
//     let vector = stable::Vector {
//         id: 0,
//         x: 0.1,
//         y: 0.2,
//         z: [0.3],
//     };
//     let extrude_hub = active::Extrude {
//         block: Grc::new(vector).into(),
//     }
//     .hub();
//     let extrude = extrude_hub.base().await?;
//     println!("extrude: {:?}", extrude.flat());
//     Ok(())
// }
