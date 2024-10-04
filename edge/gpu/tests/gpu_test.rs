//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use graph::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn gpu<'a>() -> dom::Result<Gpu<'a>> {
    let canvas = Window::new()?
        .document()?
        .body()?
        .element("canvas")?
        .canvas()?;
    canvas.gpu().await
}

fn vertex_buffer(gpu: &Gpu, size: u64) -> gpu::Result<Buffer> {
    gpu.buffer().size(size).usage(BufferUsages::VERTEX).make()
}

fn storage_buffer(gpu: &Gpu, size: u64) -> gpu::Result<Buffer> {
    gpu.buffer()
        .size(size)
        .usage(BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC)
        .make()
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
async fn make_buffer() -> dom::Result<()> {
    let gpu = gpu().await?;
    vertex_buffer(&gpu, 36)?;
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("compute.wgsl"));
    let pipe = gpu.compute().shader(&shader).entry("main").make()?;
    let size = 36;
    let storage = storage_buffer(&gpu, size)?;
    let stage = gpu.buffer().label("stage").size(size).map_read().make()?;
    storage.writer().data(basic_vec_u32()).make()?.act().await?;
    let bind_group = gpu.bind_group().pipeline(&pipe).entry(0, &storage).make()?;
    let mut encoder = gpu.encoder();
    encoder
        .compute()
        .pipeline(&pipe)
        .bind_group(0, &bind_group, &[])
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
    let gpu = gpu().await?;
    let surface = gpu.surface();
    let shader = gpu.shader(wgpu::include_wgsl!("triangle.wgsl"));
    let vertex = gpu.vertex(&shader).entry("vs_main").make()?;
    let fragment = surface.fragment(&shader).entry("fs_main").make()?;
    let view = surface.view();
    let attachment = gpu.attachment().view(&view).make()?;
    let pipe = gpu.render().vertex(vertex).fragment(fragment).make()?;
    let mut encoder = gpu.encoder();
    // encoder.render(descriptor);
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