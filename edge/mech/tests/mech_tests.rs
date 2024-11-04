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
async fn nurbs() -> dom::Result<()> {
    let gpu = gpu().await?;
    let mech = Mech::new(gpu.clone())?;
    let count = 5;
    let warp = gpu.hedge(vec![-1.5_f32, -2.5, 0.5, 2.4, 1.4, 0.8])?;
    //                         6 knots                      3 weights
    let nurbs = gpu.hedge(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])?;
    let flow_hedge = gpu.hedge(vec![0_u32, 0, 1, 2])?;
    let flow = mech.flow().matrix(3, flow_hedge).build()?;
    let shape = mech
        .shape(2)
        .warp(warp)
        .nurbs(3, nurbs)
        .flow(flow)
        .build()?;
    let plot = mech.chart(shape).grid(count)?.base().await?;
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

#[wasm_bindgen_test]
async fn draw_nurbs() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    let count = 40;
    #[rustfmt::skip]
    let flow2: Vec<u32> = vec![
        0,   1, 3,
    ];
    #[rustfmt::skip]
    let flow3: Vec<u32> = vec![
        0,   3, 4, 5, 
        1,   5, 6, 7,
    ];
    let flow = mech
        .flow()
        .matrix(2, gpu.hedge(flow2)?)
        .matrix(3, gpu.hedge(flow3)?)
        .build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .nurbs(2, gpu.hedge(nurbs2())?)
        .nurbs(3, gpu.hedge(nurbs3())?)
        .flow(flow)
        .build()?;
    let plot = mech.chart(shape).grid(count)?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_surface() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    let count = 40;
    #[rustfmt::skip]
    let flow2: Vec<u32> = vec![
        0,   1, 3,
        0,   7, 5,
    ];
    #[rustfmt::skip]
    let flow3: Vec<u32> = vec![
        0,   3, 4, 5, 
        1,   5, 6, 7,
    ];
    let rank1 = mech
        .flow()
        .matrix(2, gpu.hedge(flow2)?)
        // .matrix(3, gpu.hedge(flow3)?)
        .build()?;
    #[rustfmt::skip]
    let flow2: Vec<u32> = vec![
        0,   0, 1,
    ];
    let rank2 = mech.flow().matrix(2, gpu.hedge(flow2)?).build()?;
    let shape = mech
        .shape(2)
        .warp(gpu.hedge(warp2())?)
        .nurbs(2, gpu.hedge(nurbs2())?)
        // .nurbs(3, gpu.hedge(nurbs3())?)
        .flow(rank1)
        // .flow(rank2)
        .build()?;
    let plot = mech.chart(shape).grid(count)?;
    mech.draw(plot).points().hub()?.base().await?;
    Ok(())
}
