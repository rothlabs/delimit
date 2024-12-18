use super::*;

// pub fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<Command>>> {
//     let mech = &view.mech;
//     let topic = &mech.store.topic;
//     #[rustfmt::skip]
//     let extrude2: Vec<f32> = vec![
//         // index of weft form,    index of warp shapes (control points)
//         0.,   8.,
//     ];
//     #[rustfmt::skip]
//     let spline3: Vec<f32> = vec![
//         0.,   1., 4., 3.,
//         0.,   7., 6., 5.,
//     ];
//     let flow1 = mech::flat::shape::Flow {
//         travel: Some(topic.hedge(extrude2)),
//         spline: [(3, topic.hedge(spline3))].into(),
//         ..Default::default()
//     };
//     #[rustfmt::skip]
//     let spline3: Vec<f32> = vec![
//         0.,   1., 0., 2.
//     ];
//     let flow2 = mech::flat::shape::Flow {
//         spline: [(3, topic.hedge(spline3))].into(),
//         ..Default::default()
//     };
//     let shape = mech::flat::Shape {
//         dimension: 2,
//         warp: topic.hedge(warp2()),
//         form: mech::flat::shape::Form {
//             travel: Some(form_travel(topic)),
//             spline: [form_spline2(topic), form_spline3(topic)].into(),
//             ..Default::default()
//         },
//         flows: vec![flow1],
//     };
//     let chart = mech.chart(shape).grid(40);
//     let image = view.image(&chart).points();
//     Ok(image)
// }

pub fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<Command>>> {
    let mech = &view.mech;
    let topic = &mech.store.topic;
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
    let flow1 = mech::flat::shape::Flow {
        // travel:
        spline: [(2, topic.hedge(spline2)), (3, topic.hedge(spline3))].into(),
        ..Default::default()
    };
    #[rustfmt::skip]
    let spline3: Vec<f32> = vec![
        0.,   1., 0., 2.
    ];
    let flow2 = mech::flat::shape::Flow {
        spline: [(3, topic.hedge(spline3))].into(),
        ..Default::default()
    };
    let shape = mech::flat::Shape {
        dimension: 2,
        warp: topic.hedge(warp2()),
        form: mech::flat::shape::Form {
            spline: [form_spline2(topic), form_spline3(topic)].into(),
            ..Default::default()
        },
        flows: vec![flow1, flow2],
    };
    let chart = mech.chart(shape).grid(40);
    let image = view.image(&chart).points();
    Ok(image)
}

// fn form_travel(topic: &Shelf) -> mech::flat::shape::form::Travel {
//     mech::flat::shape::form::Travel {
//         extrude: Some(topic.hedge(extrude2())),
//     }
// }

fn form_spline2(topic: &Shelf) -> (u32, mech::flat::shape::form::Spline) {
    (
        2,
        mech::flat::shape::form::Spline {
            nurbs: Some(topic.hedge(nurbs2())),
            ..Default::default()
        },
    )
}

fn form_spline3(topic: &Shelf) -> (u32, mech::flat::shape::form::Spline) {
    (
        3,
        mech::flat::shape::form::Spline {
            nurbs: Some(topic.hedge(nurbs3())),
            ..Default::default()
        },
    )
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

// #[rustfmt::skip]
// fn extrude2() -> Vec<f32> {
//     vec![
//         0.707, 0.707, 
//     ]
// }

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

// pub fn draw_nurbs_surface(view: &mech::View) -> Result<Hub<Grc<Command>>> {
//     let mech = &view.mech;
//     let topic = &mech.store.topic;
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
//     let flow1 = mech::flat::shape::Flow {
//         // travel:
//         spline: [(2, topic.hedge(spline2)), (3, topic.hedge(spline3))].into(),
//         ..Default::default()
//     };
//     #[rustfmt::skip]
//     let spline3: Vec<f32> = vec![
//         0.,   1., 0., 2.
//     ];
//     let flow2 = mech::flat::shape::Flow {
//         spline: [(3, topic.hedge(spline3))].into(),
//         ..Default::default()
//     };
//     let shape = mech::flat::Shape {
//         dimension: 2,
//         warp: topic.hedge(warp2()),
//         form: mech::flat::shape::Form {
//             spline: [form_spline2(topic), form_spline3(topic)].into(),
//             ..Default::default()
//         },
//         flows: vec![flow1],
//     };
//     let chart = mech.chart(shape).grid(40);
//     let image = view.image(&chart).points();
//     Ok(image)
// }

// fn form_travel2(topic: &Shelf) -> (u32, mech::flat::shape::form::Travel) {
//     (
//         2,
//         mech::flat::shape::form::Travel {
//             extrude: Some(topic.hedge(extrude2())),
//         },
//     )
// }

// fn form_spline2(topic: &Shelf) -> (u32, mech::flat::shape::form::Spline) {
//     (
//         2,
//         mech::flat::shape::form::Spline {
//             nurbs: Some(topic.hedge(nurbs2())),
//             ..Default::default()
//         },
//     )
// }

// fn form_spline3(topic: &Shelf) -> (u32, mech::flat::shape::form::Spline) {
//     (
//         3,
//         mech::flat::shape::form::Spline {
//             nurbs: Some(topic.hedge(nurbs3())),
//             ..Default::default()
//         },
//     )
// }

// #[rustfmt::skip]
// fn warp2() -> Vec<f32> {
//     vec![
//         -0.9, -0.9,
//         -0.9, 0.,
//         -0.9, 0.9,
//         0., 0.9,
//         0.9, 0.9,
//         0.9, 0.,
//         0.9, -0.9,
//         0., -0.9,
//         0., 0.,
//     ]
// }

// #[rustfmt::skip]
// fn extrude2() -> Vec<f32> {
//     vec![
//         0.707, 0.707,
//     ]
// }

// #[rustfmt::skip]
// fn nurbs2() -> Vec<f32> {
//     vec![
//         0., 0., 1., 1.,    1., 1.,
//     ]
// }

// #[rustfmt::skip]
// fn nurbs3() -> Vec<f32> {
//     vec![
//         0., 0., 0., 1., 1., 1.,    1., (2.0_f32).sqrt() / 2., 1.,
//         0., 0., 0., 1., 1., 1.,    1., 1.5, 1.,
//     ]
// }
