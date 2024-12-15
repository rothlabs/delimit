use super::*;

#[tokio::test]
async fn extrude() -> Result<()> {
    let vector = stable::Vector {
        id: 0,
        data: [0.1, 0.2, 0.3],
        wow: [],
    };
    let extrude_hub = active::Extrude {
        block: Grc::new(vector).into(),
    }
    .hub();
    let extrude = extrude_hub.base().await?;
    println!("extrude: {:?}", extrude.flat());
    Ok(())
}
