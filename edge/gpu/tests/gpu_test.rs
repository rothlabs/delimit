//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use graph::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> dom::Result<Element> {
    Window::new()?.document()?.body()
}

async fn gpu<'a>() -> dom::Result<Gpu<'a>> {
    let canvas = body()?.element("canvas")?.canvas()?;
    canvas.gpu().await
}

async fn gpu_with_canvas<'a>() -> dom::Result<Gpu<'a>> {
    let canvas = body()?.stem("canvas")?.canvas()?;
    canvas.gpu().await
}

#[rustfmt::skip]
fn basic_vec_u32() -> Vec<u32> {
    vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ]
}

// Tests ///////////////////////////////

#[wasm_bindgen_test]
async fn make_vertex_buffer() -> dom::Result<()> {
    let gpu = gpu().await?;
    gpu.buffer(1024).usage(BufferUsages::VERTEX).make()?;
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("compute.wgsl"));
    let pipe = gpu.compute_pipe(&shader).entry("main").make()?;
    let size = 36;
    let storage = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).label("stage").map_read()?;
    storage.writer(basic_vec_u32()).make()?.act().await?;
    let bind_group = gpu.bind_group().pipe(&pipe).entry(0, &storage).make()?;
    let mut encoder = gpu.encoder();
    encoder
        .compute()
        .pipe(&pipe)
        .bind(0, &bind_group, &[])
        .debug("compute collatz iterations")
        .dispatch(9, 1, 1);
    encoder
        .copy_buffer(&storage)
        .destination(&stage)
        .size(size)
        .submit();
    let out: Vec<u32> = stage.reader().hub()?.base().await?;
    assert_eq!(out, vec![2, 4, 6, 8, 10, 12, 14, 16, 18]);
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_triangle() -> dom::Result<()> {
    // setup:
    let gpu = gpu_with_canvas().await?;
    let surface = gpu.surface();
    let shader = gpu.shader(wgpu::include_wgsl!("triangle.wgsl"));
    let vertex = gpu.vertex(&shader).entry("vs_main").make()?;
    let fragment = surface.fragment(&shader).entry("fs_main").make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    // render loop:
    let view = surface.view();
    let attachments = gpu.attachment(&view).list()?;
    let pass = gpu.render_pass(&attachments).make()?;
    let mut encoder = gpu.encoder();
    encoder.render(&pass).pipe(&pipe).draw(0..3, 0..1);
    encoder.submit();
    Ok(())
}

// #[rustfmt::skip]
// fn basic_f32() -> Vec<f32> {
//     vec![
//         1., 2., 3.,
//         4., 5., 6.,
//         7., 8., 9.,
//     ].into()
// }
