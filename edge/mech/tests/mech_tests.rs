#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use mech::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> dom::Result<Element> {
    Window::new()?.document()?.body()
}

async fn gpu_with_canvas<'a>() -> dom::Result<(Gpu, Surface<'a>)> {
    let canvas = body()?.stem("canvas")?.canvas()?;
    canvas.gpu().await
}

#[wasm_bindgen_test]
async fn nurbs() -> dom::Result<()> {
    let (gpu, _) = gpu_with_canvas().await?;
    let mech = Mech::new(gpu.clone())?;
    let count = 5;
    //                        6 knots                      3 weights
    let plan = gpu.hedge(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])?;
    let index = gpu.hedge(vec![0_u32, 1, 2])?;
    let control = gpu.hedge(vec![-1.5_f32, -2.5, 0.5, 2.4, 1.4, 0.8])?;
    let shape = mech.shape(Rule::Nurbs(3))
        .plan(plan)
        .index(index)
        .control(Control::Hedge(control))
        .build()?;
    // let shape = ShapeBuilder::default()
    //     .gpu(gpu.clone())
    //     .rule(Rule::Nurbs(3))
    //     .plan(plan)
    //     .index(index)
    //     .control(Control::Hedge(control))
    //     .build()?;
    let plot = plot::GridBuilder::default()
        .shape(shape)
        .count(count)
        .hub()?
        .base()
        .await?;
    let size = plot.buffer.base().await?.size();
    let stage = gpu.buffer(size).map_read()?;
    let out: Vec<f32> = gpu
        .reader(plot.buffer)
        .root(plot.root)
        .stage(stage)
        .hub()?
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