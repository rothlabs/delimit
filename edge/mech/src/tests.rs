use super::*;
// use stable::*;

#[tokio::test]
async fn extrude() -> Result<()> {
    // points
    let middle_center = stable::Vector::new(0., 0., []);
    let middle_left = stable::Vector::new(-0.9, 0., []);
    let top_right = stable::Vector::new(0.9, 0.9, []);
    let top_center = stable::Vector::new(0., 0.9, []);
    let bottom_center = stable::Vector::new(0., -0.9, []);
    let bottom_right = stable::Vector::new(0.9, -0.9, []);
    let middle_right = stable::Vector::new(0.9, 0., []);

    // vectors
    let extrude = stable::Vector::new(0.8, 0.8, []);
    let knots = stable::block::spline::Knots::new([0., 0., 0., 1., 1., 1.], []);

    let differ = Grc::new(stable::block::Differ {
        form: stable::block::differ::Form::Extrude(extrude),
        stem: stable::block::Form::Point(middle_center),
    });

    let spline0 = Grc::new(stable::block::Spline {
        form: stable::block::spline::Form::Basis(knots.clone()),
        stems_a: [middle_left.into(), top_right.into(), top_center.into()],
        stems_b: [],
    });

    let spline1 = Grc::new(stable::block::Spline {
        form: stable::block::spline::Form::Basis(knots.clone()),
        stems_a: [bottom_center.into(), bottom_right.into(), middle_right.into()],
        stems_b: [],
    });

    let surface = stable::block::Spline {
        form: stable::block::spline::Form::Basis(knots),
        stems_a: [spline0.into(), differ.into(), spline1.into()],
        stems_b: [],
    };

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
