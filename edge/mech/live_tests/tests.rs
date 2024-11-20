use super::*;

pub async fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<gpu::Action>>> {
    let mech = &view.mech;
    let gpu = &mech.gpu;
    #[rustfmt::skip]
    let spline2 = vec![
        0,   8, 4,
    ];
    #[rustfmt::skip]
    let spline3 = vec![
        0,   1, 4, 3, 
        0,   7, 6, 5,
    ];
    let flow1 = mech
        .flow()
        .spline(gpu.hedge(spline2)?, 2)
        .spline(gpu.hedge(spline3)?, 3)
        .build()?;
    #[rustfmt::skip]
    let spline3 = vec![
        0,   1, 0, 2
    ];
    let flow2 = mech.flow().spline(gpu.hedge(spline3)?, 3).build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .nurbs(gpu.hedge(nurbs2())?, 2)
        .nurbs(gpu.hedge(nurbs3())?, 3)
        .flow(flow1)
        .flow(flow2)
        .build()?;
    let plot = mech.plot(shape).grid(30).hub()?;
    let drawing = view.plot(&plot).points().hub()?;
    Ok(drawing)
}

#[rustfmt::skip]
fn warp2() -> Vec<f32> {
    vec![
        -0.9, -0.9,
        -0.9, 0.,
        -0.9, 0.9,
        0., 0.9, 
        0.9, 0.9,
        0.9, 0.,
        0.9, -0.9, 
        0., -0.9,
        0., 0.,
    ]
}

#[rustfmt::skip]
fn nurbs2() -> Vec<f32> {
    vec![
        0., 0., 1., 1.,    1., 1.,
    ]
}

#[rustfmt::skip]
fn nurbs3() -> Vec<f32> {
    vec![
        0., 0., 0., 1., 1., 1.,    1., (2.0_f32).sqrt() / 2., 1.,
        0., 0., 0., 1., 1., 1.,    1., 1.5, 1.,
    ]
}

// let drawing = view.plot(&plot).points().hub()?;
// let drawing2 = view.plot(plot).points().hub()?;
// let mut wow = JoinBuilder::default();
// wow.field(drawing);
// wow.field(drawing2);
// Ok(wow.hub()?)

// #[derive(Builder, Gate, Back, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into))]
// pub struct NurbsSurface {
//     #[back(skip)]
//     view: View,

// }

// impl Solve for NurbsSurface {
//     type Base = Mutation;
//     async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//         // println!("app act");
//         // let displays = self.displays.base()?;

//         Ok(Mutation.into())
//     }
// }
