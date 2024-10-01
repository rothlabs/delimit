//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use graph::*;
use dom::*;
use gpu::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

async fn gpu() -> dom::Result<Gpu> {
    let canvas = Window::new()?
        .document()?
        .body()?
        .element("canvas")?
        .canvas()?;
    canvas.gpu().await
}

fn vertex_buffer(gpu: &Gpu, size: u64) -> dom::Result<Grc<wgpu::Buffer>> {
    let buffer = gpu.buffer()
        .size(size)
        .usage(wgpu::BufferUsages::VERTEX)
        .make()?;
    Ok(buffer)
}

fn storage_buffer_with_writer(gpu: &Gpu) -> dom::Result<(Grc<wgpu::Buffer>, Node<BufferWriter>)> {
    let buffer = gpu.buffer()
        .size(36)
        .usage(wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC)
        .make()?;
    #[rustfmt::skip]
    let data: Vec<f32> = vec![
        0., 0., 0.,
        1., 0., 0.,
        0., 1., 0., 
    ];
    let writer = gpu.buffer_writer().buffer(buffer.clone()).data(data).node()?;
    Ok((buffer, writer))
}

// Tests ///////////////////////////////

#[wasm_bindgen_test]
async fn make_buffer() -> dom::Result<()> {
    let gpu = gpu().await?;
    vertex_buffer(&gpu, 36)?;
    Ok(())
}

#[wasm_bindgen_test]
async fn make_buffer_writer() -> dom::Result<()> {
    let gpu = gpu().await?;
    storage_buffer_with_writer(&gpu)?;
    Ok(())
}

#[wasm_bindgen_test]
async fn make_compute_pipeline() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("compute.wgsl"));
    let pipe = gpu.compute().shader(&shader).entry("main").make()?;
    let layout = pipe.get_bind_group_layout(0);
    // let bind = gpu.bind_group().layout(&layout).entry(0, resource).make()?;
    Ok(())
}


