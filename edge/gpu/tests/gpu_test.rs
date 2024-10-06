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
async fn draw_triangle() -> dom::Result<()> {
    // setup:
    let gpu = gpu_with_canvas().await?;
    let surface = gpu.surface();
    let shader = gpu.shader(wgpu::include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").surface()?;
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

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("collatz.wgsl"));
    let pipe = gpu.compute_pipe(&shader).entry("main").make()?;
    let size = 36;
    let storage = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).label("stage").map_read()?;
    storage.writer(basic_vec_u32()).make()?.act().await?;
    let bind = gpu.bind().pipe(&pipe).entry(0, &storage).make()?;
    let mut encoder = gpu.encoder();
    encoder
        .compute()
        .pipe(&pipe)
        .bind(0, &bind, &[])
        .debug("compute collatz iterations")
        .dispatch(9, 1, 1);
    encoder
        .copy_buffer(&storage)
        .destination(&stage)
        .size(size)
        .submit();
    let out: Vec<u32> = stage.reader().hub()?.base().await?;
    assert_eq!(out, vec![0, 1, 7, 2, 5, 8, 16, 3, 19]);
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_nurbs() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("nurbs.wgsl"));
    let count = 128;
    let size = 4 * count as u64;
    let config = gpu.unifrom_buffer(&[count]);
    let basis = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).map_read()?;

    let config_bind = gpu.uniform().make()?;
    let config_entry = gpu.bind_entry(0, config_bind).compute()?;
    let basis_bind = gpu.storage(false).make()?;
    let basis_entry = gpu.bind_entry(1, basis_bind).compute()?;
    let bind_layout = gpu.bind_layout(&[config_entry, basis_entry]).make()?;
    let bind = gpu
        .bind()
        .layout(&bind_layout)
        .entry(0, &config)
        .entry(1, &basis)
        .make()?;

    let pipe_layout = gpu.pipe_layout().bind_group_layouts(&[&bind_layout]).make()?;
    let pipe = gpu.compute_pipe(&shader).layout(&pipe_layout).entry("main").make()?;

    let mut encoder = gpu.encoder();
    encoder
        .compute()
        .pipe(&pipe)
        .bind(0, &bind, &[])
        .dispatch(count, 1, 1);
    encoder
        .copy_buffer(&basis)
        .destination(&stage)
        .size(size)
        .submit();
    let out: Vec<f32> = stage.reader().hub()?.base().await?;
    console_log!("nurbs: {:?}, length: {}", out, out.len());
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
