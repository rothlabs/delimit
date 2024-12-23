use super::*;
use stable::*;
// use stable::block::*;

#[tokio::test]
async fn extrude() -> Result<()> {
    // Vector Leaf
    let extrude = Vector::new(0.8, 0.8, []);
    let knots6 = block::spline::Knots::new([0., 0., 0., 1., 1., 1.], []);
    let knots8 = block::spline::Knots::new([0., 0., 0., 1., 1., 1.], [(0., 1.)]);

    // Point Leaf, rank 0
    let middle_center = Vector::new(0., 0., []);
    let middle_left = Vector::new(-0.9, 0., []);
    let top_right = Vector::new(0.9, 0.9, []);
    let top_center = Vector::new(0., 0.9, []);
    let bottom_center = Vector::new(0., -0.9, []);
    let bottom_right = Vector::new(0.9, -0.9, []);
    let middle_right = Vector::new(0.9, 0., []);

    // Block, rank 1
    let differ: Grc<Block<Vector<f64, 0>, f64, 0>> = Grc::new(
        block::Differ {
            kind: block::differ::Kind::Extrude(extrude),
            stem: middle_center,
        }
        .into(),
    );

    // Block, rank 1
    let spline3: Grc<Block<Vector<f64, 0>, f64, 0>> = Grc::new(
        block::Spline {
            kind: block::spline::Kind::Basis(knots6.clone()),
            stems_a: [middle_left.clone(), top_right, top_center],
            stems_b: [],
        }
        .into(),
    );

    // Block, rank 1
    let spline4: Grc<Block<Vector<f64, 0>, f64, 0>> = Grc::new(
        block::Spline {
            kind: block::spline::Kind::Basis(knots8),
            stems_a: [bottom_center, bottom_right, middle_right],
            stems_b: [middle_left],
        }
        .into(),
    );

    // Block, rank 2
    let surface: Grc<Block<Block<Vector<f64, 0>, f64, 0>, f64, 0>> = Grc::new(
        block::Spline {
            kind: block::spline::Kind::Basis(knots6),
            stems_a: [spline3.clone(), differ, spline4],
            stems_b: [],
        }
        .into(),
    );

    let shape = Shape {
        block: surface,
        bound: spline3,
    };

    shape.flat();

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
