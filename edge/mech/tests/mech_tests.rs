#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use mech::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> dom::Result<Element> {
    Window::new()?.document()?.body()
}

async fn gpu() -> dom::Result<Gpu> {
    let canvas = body()?.element("canvas")?.canvas()?;
    canvas.gpu().await
}

async fn gpu_with_canvas<'a>() -> dom::Result<Gpu> {
    let canvas = body()?.stem("canvas")?.canvas()?;
    let gpu = canvas.gpu().await?;
    gpu.display.resize(300, 300).await?;
    Ok(gpu)
}

#[wasm_bindgen_test]
async fn nurbs_curve() -> dom::Result<()> {
    let gpu = gpu().await?;
    let mech = Mech::new(gpu.clone())?;
    let count = 5;
    let warp = gpu.hedge(vec![-1.5_f32, -2.5, 0.5, 2.4, 1.4, 0.8])?;
    //                         6 knots                      3 weights
    let nurbs = gpu.hedge(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])?;
    let flow_hedge = gpu.hedge(vec![0_u32, 0, 1, 2])?;
    let flow = mech.flow().spline(flow_hedge, 3).build()?;
    let shape = mech
        .shape(2)
        .warp(warp)
        .nurbs(nurbs, 3)
        .flow(flow)
        .build()?;
    let plot = mech.chart(shape).grid(count).hub()?.base().await?;
    let out: Vec<f32> = gpu
        .reader(plot.hedge.buffer)
        .root(plot.hedge.root)
        .staged()?
        .base()
        .await?;
    assert_eq!(
        out,
        vec![
            -1.5,
            -2.5,
            4.0,
            9.8,
            -0.56875,
            -0.45624995,
            3.45,
            6.55,
            0.225,
            0.77500004,
            2.9,
            3.3,
            0.88124996,
            1.19375,
            2.35,
            0.049999923,
            1.4,
            0.8,
            1.8,
            -3.2000003
        ]
    );
    Ok(())
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
fn extrude2() -> Vec<f32> {
    vec![
        -0.9, -0.25,
        -0.9, 0.8,
    ]
}

#[rustfmt::skip]
fn revolve2() -> Vec<f32> {
    vec![
        4.
    ]
}

#[rustfmt::skip]
fn nurbs2() -> Vec<f32> {
    vec![
        0., 0., 1., 1.,    1., 1.,
    ]
}

#[rustfmt::skip]
fn basis3() -> Vec<f32> {
    vec![
        0., 0.2, 0.4, 0.6, 0.8, 1., 
    ]
}

#[rustfmt::skip]
fn nurbs3() -> Vec<f32> {
    vec![
        0., 0., 0., 1., 1., 1.,    1., (2.0_f32).sqrt() / 2., 1.,
        0., 0., 0., 1., 1., 1.,    1., 1.5, 1.,
    ]
}

#[rustfmt::skip]
fn basis4() -> Vec<f32> {
    vec![
        0., 0., 0., 0.15, 0.85, 1., 1., 1., // 0., 0.143, 0.286, 0.428, 0.571, 0.714, 0.857, 1.,
    ]
}

#[wasm_bindgen_test]
async fn draw_curves() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    #[rustfmt::skip]
    let travel: Vec<u32> = vec![
        0,   8, 
    ];
    #[rustfmt::skip]
    let spline2: Vec<u32> = vec![
        0,   8, 6,
    ];
    #[rustfmt::skip]
    let spline3: Vec<u32> = vec![
        0,   1, 0, 8, 
        1,   1, 8, 3, 
        2,   7, 6, 5,
    ];
    #[rustfmt::skip]
    let spline4: Vec<u32> = vec![
        0,   3, 8, 4, 5, 
    ];
    let flow = mech
        .flow()
        .travel(gpu.hedge(travel)?)
        .spline(gpu.hedge(spline2)?, 2)
        .spline(gpu.hedge(spline3)?, 3)
        .spline(gpu.hedge(spline4)?, 4)
        .build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .extrude(gpu.hedge(extrude2())?)
        .nurbs(gpu.hedge(nurbs2())?, 2)
        .nurbs(gpu.hedge(nurbs3())?, 3)
        .basis(gpu.hedge(basis3())?, 3)
        .basis(gpu.hedge(basis4())?, 4)
        .flow(flow)
        .build()?;
    let plot = mech.chart(shape).grid(30).hub()?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_nurbs_surface() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    #[rustfmt::skip]
    let spline2: Vec<u32> = vec![
        0,   8, 4,
    ];
    #[rustfmt::skip]
    let spline3: Vec<u32> = vec![
        0,   1, 4, 3, 
        0,   7, 6, 5,
    ];
    let flow1 = mech
        .flow()
        .spline(gpu.hedge(spline2)?, 2)
        .spline(gpu.hedge(spline3)?, 3)
        .build()?;
    #[rustfmt::skip]
    let spline3: Vec<u32> = vec![
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
    let plot = mech.chart(shape).grid(30).hub()?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_extrusion_surface() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    #[rustfmt::skip]
    let travel: Vec<u32> = vec![
        1,   0, 
    ];
    #[rustfmt::skip]
    let spline3: Vec<u32> = vec![
        0,   7, 6, 5,
    ];
    let flow1 = mech.flow().spline(gpu.hedge(spline3)?, 3).build()?;
    let flow2 = mech.flow().travel(gpu.hedge(travel)?).build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .extrude(gpu.hedge(extrude2())?)
        .nurbs(gpu.hedge(nurbs3())?, 3)
        .flow(flow1)
        .flow(flow2)
        .build()?;
    let plot = mech.chart(shape).grid(30).hub()?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_revolve_surface() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    #[rustfmt::skip]
    let orient: Vec<u32> = vec![
        0,   0,
    ];
    #[rustfmt::skip]
    let spline3: Vec<u32> = vec![
        0,   8, 3, 2,
    ];
    let flow1 = mech.flow().spline(gpu.hedge(spline3)?, 3).build()?;
    let flow2 = mech.flow().orient(gpu.hedge(orient)?).build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .revolve(gpu.hedge(revolve2())?)
        .basis(gpu.hedge(basis3())?, 3)
        .flow(flow1)
        .flow(flow2)
        .build()?;
    let plot = mech.chart(shape).grid(16).count(45).hub()?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}
