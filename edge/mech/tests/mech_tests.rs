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
    let size = 9 * 4;
    let storage = gpu.buffer(size).storage_copy()?;
    let mutator = gpu
        .writer(storage.clone())
        ////////// 6 knots                      3 weights
        .data(vec![0.0_f32, 0., 0., 1., 1., 1., 1., 1., 1.])
        .hub()?;
    let hedge = Hedge {
        buffer: storage.into(),
        mutator,
    };
    let shape = ShapeBuilder::default()
        .gpu(gpu.clone())
        .rule(Rule::Nurbs(3))
        .plan(Table::Hedge(hedge))
        .control(Control::Shape(vec![]))
        .build()?;
    let plot = plot::GridBuilder::default()
        .shape(shape)
        .count(count)
        .hub()?
        .base()
        .await?;
    let stage = gpu.buffer(count as u64 * size * 2 / 3).map_read()?;
    let out: Vec<f32> = gpu
        .reader(plot.buffer)
        .mutator(plot.mutator)
        .stage(stage)
        .hub()?
        .base()
        .await?;
    assert_eq!(
        out,
        vec![
            1.0, 0.0, 0.0, -2.0, 2.0, 0.0, 0.5625, 0.375, 0.0625, -1.5, 1.0, 0.5, 0.25, 0.5, 0.25,
            -1.0, 0.0, 1.0, 0.0625, 0.375, 0.5625, -0.5, -1.0, 1.5, 0.0, 0.0, 1.0, 0.0, -2.0, 2.0
        ]
    );
    Ok(())
}

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
