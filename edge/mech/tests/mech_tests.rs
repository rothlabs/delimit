#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use mech::*;
// use graph::*;
use wasm_bindgen_test::*;
// use wgpu::*;

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
    let count = 5;
    //                        6 knots                      3 weights
    let plan = gpu.hedge(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])?;
    let index = gpu.hedge(vec![0_u32, 1, 2])?;
    let control = gpu.hedge(vec![-1.5_f32, -2.5, 0.5, 2.4, 1.4, 0.8])?;
    let shape = ShapeBuilder::default()
        .gpu(gpu.clone())
        .rule(Rule::Nurbs(3))
        .plan(plan)
        .index(index)
        .control(Control::Hedge(control))
        .build()?;
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

// let plan_buffer = gpu.buffer(size).storage_copy()?;
// let plan_writer = gpu
//     .writer(plan_buffer.clone())
//     ////////// 6 knots                      3 weights
//     .data(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])
//     .hub()?;
// let plan = Hedge {
//     buffer: plan_buffer.into(),
//     mutator: plan_writer,
// };

// #[wasm_bindgen_test]
// async fn nurbs() -> dom::Result<()> {
//     let (gpu, _) = gpu_with_canvas().await?;
//     let shader = gpu.shader(include_wgsl!("../src/shader/nurbs.wgsl"));
//     let order = 3;
//     let count = 64;
//     let size = 4 * count as u64;
//     let config = gpu.buffer_uniform(&[order, count]);
//     let knots = gpu.buffer(size).storage_copy()?;
//     let weights = gpu.buffer(size).storage_copy()?;
//     let basis = gpu.buffer(size).storage_copy()?;
//     let stage = gpu.buffer(size).map_read()?;
//     let config_entry = gpu.uniform().entry(0)?.compute()?;
//     let knots_entry = gpu.storage(false).entry(1)?.compute()?;
//     let weights_entry = gpu.storage(false).entry(2)?.compute()?;
//     let basis_entry = gpu.storage(false).entry(3)?.compute()?;
//     let bind_layout = gpu
//         .bind_layout(&[config_entry, knots_entry, weights_entry, basis_entry])
//         .make()?;
//     let bind = gpu
//         .bind()
//         .layout(&bind_layout)
//         .entry(0, &config)
//         .entry(1, &knots)
//         .entry(2, &weights)
//         .entry(3, &basis)
//         .make()?;
//     let pipe_layout = gpu.pipe_layout(&[&bind_layout]).make()?;
//     let pipe = shader.compute("main").layout(&pipe_layout).make()?;
//     let mut encoder = gpu.encoder();
//     encoder
//         .compute()
//         .pipe(&pipe)
//         .bind(0, &bind, &[])
//         .dispatch(count, 1, 1);
//     encoder
//         .copy_buffer(&basis)
//         .destination(&stage)
//         .size(size)
//         .submit();
//     let out: Vec<f32> = gpu.reader(stage).hub()?.base().await?;
//     assert_eq!(
//         out,
//         vec![
//             0.0, 0.06666667, 0.13333334, 0.20000002, 0.26666668, 0.33333334, 0.40000004, 0.4666667,
//             0.53333336, 0.6, 0.6666667, 0.73333335, 0.8000001, 0.86666673, 0.9333334, 1.0
//         ]
//     );
//     Ok(())
// }
