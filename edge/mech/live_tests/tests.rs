use super::*;

pub fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<gpu::Action>>> {
    let mech = &view.mech;
    let gpu = &mech.gpu;
    #[rustfmt::skip]
    let spline2: Vec<f32> = vec![
        // index of weft form,    index of warp shapes (control points)
        0.,   8., 4.,
    ];
    #[rustfmt::skip]
    let spline3: Vec<f32> = vec![
        0.,   1., 4., 3., 
        0.,   7., 6., 5.,
    ];
    let flow1 = Flow {
        splines: vec![
            flow::Spline {
                order: 2,
                hedge: gpu.hedge(spline2),
            },
            flow::Spline {
                order: 3,
                hedge: gpu.hedge(spline3),
            },
        ],
        ..Default::default()
    };
    #[rustfmt::skip]
    let spline3: Vec<f32> = vec![
        0.,   1., 0., 2.
    ];
    let flow2 = Flow {
        splines: vec![flow::Spline {
            order: 3,
            hedge: gpu.hedge(spline3),
        }],
        ..Default::default()
    };
    let shape = Shape {
        dimension: 2,
        warp: gpu.hedge(warp2()),
        form: Form {
            splines: vec![form_spline2(gpu), form_spline3(gpu)],
            ..Default::default()
        },
        flows: vec![flow1, flow2],
    };
    let chart = mech.chart(shape).grid(40);
    let drawing = view.image(&chart).points();
    Ok(drawing)
}

// fn flow1()

fn form_spline2(gpu: &Gpu) -> form::Spline {
    form::Spline {
        order: 2,
        nurbs: Some(gpu.hedge(nurbs2())),
        ..Default::default()
    }
}

fn form_spline3(gpu: &Gpu) -> form::Spline {
    form::Spline {
        order: 3,
        nurbs: Some(gpu.hedge(nurbs3())),
        ..Default::default()
    }
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

// pub fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<gpu::Action>>> {
//     let mech = &view.mech;
//     let gpu = &mech.gpu;
//     #[rustfmt::skip]
//     let spline2: Vec<f32> = vec![
//         // index of weft form,    index of warp shapes (control points)
//         0.,   8., 4.,
//     ];
//     #[rustfmt::skip]
//     let spline3: Vec<f32> = vec![
//         0.,   1., 4., 3.,
//         0.,   7., 6., 5.,
//     ];
//     let flow1 = mech
//         .flow()
//         .spline(gpu.hedge(spline2)?, 2)
//         .spline(gpu.hedge(spline3)?, 3)
//         .build()?;
//     #[rustfmt::skip]
//     let spline3: Vec<f32> = vec![
//         0.,   1., 0., 2.
//     ];
//     let flow2 = mech.flow().spline(gpu.hedge(spline3)?, 3).build()?;
//     // let shape = Shape {
//     //     dimension: 2,
//     //     warp: gpu.hedge(warp2())?,
//     //     form: Form {
//     //         splines: vec![None, None, Some(Spline{})]
//     //         ..Default::default()
//     //     },
//     // };
//     let shape = mech
//         .shape(2)
//         .warp(gpu.hedge(warp2())?)
//         .nurbs(gpu.hedge(nurbs2())?, 2)
//         .nurbs(gpu.hedge(nurbs3())?, 3)
//         .flow(flow1)
//         .flow(flow2)
//         .build()?;
//     let plot = mech.chart(shape).grid(40);
//     let drawing = view.plot(&plot).points().hub()?;
//     Ok(drawing)
// }

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
