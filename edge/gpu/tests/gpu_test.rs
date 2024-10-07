//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
use graph::*;
use wasm_bindgen_test::*;
use wgpu::vertex_attr_array;

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
    let targets = surface.targets();
    let shader = gpu.shader(wgpu::include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    // render:
    let view = surface.view();
    let attachments = gpu.attachment(&view).list()?;
    let pass = gpu.render_pass(&attachments).make()?;
    let mut encoder = gpu.encoder();
    encoder.render(&pass).pipe(&pipe).draw(0..3, 0..1);
    encoder.submit();
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_lines() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let surface = gpu.surface();
    let targets = surface.targets();
    let shader = gpu.shader(wgpu::include_wgsl!("line.wgsl"));
    let prim = gpu.lines().make()?;
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let buffers = gpu.vertex_layout(24).attributes(&attribs).list()?;
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).primitive(prim).make()?;
    #[rustfmt::skip]
    let buffer = gpu.buffer_vertex(&vec![
        // pos           color
        -0.9_f32, 0.,    1., 0., 0., 0.,
        0.9, -0.1,       0., 1., 0., 0.,
        -0.5, -0.5,      0., 0., 1., 0.,
        0.5, 0.5,        0., 0., 0., 0.,
    ]);
    let view = surface.view();
    let attachments = gpu.attachment(&view).list()?;
    let pass = gpu.render_pass(&attachments).make()?;
    let mut encoder = gpu.encoder();
    encoder.render(&pass).pipe(&pipe).vertex(0, buffer.slice(..)).draw(0..4, 0..1);
    encoder.submit();
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("collatz.wgsl"));
    let pipe = shader.compute("main").make()?;
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
async fn index_fraction() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("index.wgsl"));
    let count = 16;
    let size = 4 * count as u64;
    let config = gpu.buffer_uniform(&[count]);
    let basis = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).map_read()?;
    let config_entry = gpu.uniform().entry(0)?.compute()?;
    let basis_entry = gpu.storage(false).entry(1)?.compute()?;
    let bind_layout = gpu.bind_layout(&[config_entry, basis_entry]).make()?;
    let bind = gpu
        .bind()
        .layout(&bind_layout)
        .entry(0, &config)
        .entry(1, &basis)
        .make()?;
    let pipe_layout = gpu.pipe_layout(&[&bind_layout]).make()?;
    let pipe = shader.compute("main").layout(&pipe_layout).make()?;
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
    assert_eq!(
        out,
        vec![
            0.0, 0.06666667, 0.13333334, 0.20000002, 0.26666668, 0.33333334, 0.40000004, 0.4666667,
            0.53333336, 0.6, 0.6666667, 0.73333335, 0.8000001, 0.86666673, 0.9333334, 1.0
        ]
    );
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
