//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::*;
use gpu::*;
// use graph::*;
// use hub::IntoHub;
use wasm_bindgen_test::*;
use wgpu::*;

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> dom::Result<Element> {
    Window::new()?.document()?.body()
}

async fn gpu<'a>() -> dom::Result<(Gpu, gpu::Surface<'a>)> {
    let canvas = body()?.element("canvas")?.canvas()?;
    canvas.gpu().await
}

async fn gpu_with_canvas<'a>() -> dom::Result<(Gpu, gpu::Surface<'a>)> {
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

#[rustfmt::skip]
fn line_data() -> Vec<f32> {
    vec![
        // pos           color
        -0.9_f32, 0.,    1., 0., 0., 0.,
        0.9, -0.1,       0., 1., 0., 0.,
        -0.5, -0.5,      0., 0., 1., 0.,
        0.5, 0.5,        0., 0., 0., 0.,
    ]
}

// Tests ///////////////////////////////

#[wasm_bindgen_test]
async fn make_vertex_buffer() -> dom::Result<()> {
    let (gpu, _) = gpu().await?;
    gpu.buffer(1024).usage(BufferUsages::VERTEX).make()?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_triangle() -> dom::Result<()> {
    // setup:
    let (gpu, surface) = gpu_with_canvas().await?;
    let targets = surface.targets();
    let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
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

const LINE_SHADER: ShaderModuleDescriptor = include_wgsl!("../src/shader/line.wgsl");

#[wasm_bindgen_test]
async fn draw_lines() -> dom::Result<()> {
    let (gpu, surface) = gpu_with_canvas().await?;
    let targets = surface.targets();
    let shader = gpu.shader(LINE_SHADER);
    let prim = gpu.lines().make()?;
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let buffers = gpu.vertex_layout(24).attributes(&attribs).list()?;
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu
        .render_pipe(vertex)
        .fragment(fragment)
        .primitive(prim)
        .make()?;
    let buffer = gpu.buffer_vertex(&line_data());
    let view = surface.view();
    let attachments = gpu.attachment(&view).list()?;
    let pass = gpu.render_pass(&attachments).make()?;
    let mut encoder = gpu.encoder();
    encoder
        .render(&pass)
        .pipe(&pipe)
        .vertex(0, buffer.slice(..))
        .draw(0..4, 0..1);
    encoder.submit();
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_msaa_lines() -> dom::Result<()> {
    let (gpu, surface) = gpu_with_canvas().await?;
    let targets = surface.targets();
    let shader = gpu.shader(LINE_SHADER);
    let prim = gpu.lines().make()?;
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let buffers = gpu.vertex_layout(24).attributes(&attribs).list()?;
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let multi = gpu.multisample(4).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu
        .render_pipe(vertex)
        .fragment(fragment)
        .primitive(prim)
        .multisample(multi)
        .make()?;
    let buffer = gpu.buffer_vertex(&line_data());
    let view = surface.view();
    let texture_view = surface.texture().sample_count(4).view()?;
    let attachments = gpu.attachment(&texture_view).resolve_target(&view).list()?;
    let pass = gpu.render_pass(&attachments).make()?;
    let mut encoder = gpu.encoder();
    encoder
        .render(&pass)
        .pipe(&pipe)
        .vertex(0, buffer.slice(..))
        .draw(0..4, 0..1);
    encoder.submit();
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let (gpu, _) = gpu().await?;
    let shader = gpu.shader(include_wgsl!("collatz.wgsl"));
    let pipe = shader.compute("main").make()?;
    let size = 36;
    let storage = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).label("stage").map_read()?;
    //let bind = gpu.bind().pipe(&pipe).entry(0, &storage).make()?;
    let bind = gpu
        .binder()
        .pipe(pipe.clone())
        .entry(0, storage.clone())
        .hub()?;
    gpu.writer(storage.clone())
        .data(basic_vec_u32())
        .hub()?
        .base()
        .await?;
    let mutator = gpu
        .dispatcher()
        .pipe(pipe)
        .bind(bind)
        .count(9)
        // .stage(storage, stage.clone())
        .hub()?;
    let out = gpu
        .reader::<u32>(storage)
        .stage(stage)
        .mutator(mutator)
        .hub()?
        .base()
        .await?;
    assert_eq!(out, vec![0, 1, 7, 2, 5, 8, 16, 3, 19]);
    Ok(())
}

#[wasm_bindgen_test]
async fn index_fraction() -> dom::Result<()> {
    let (gpu, _) = gpu().await?;
    let shader = gpu.shader(include_wgsl!("index.wgsl"));
    let count = 16;
    let size = 4 * count as u64;
    let config = gpu.buffer_uniform(&[count]);
    let basis = gpu.buffer(size).storage_copy()?;
    let stage = gpu.buffer(size).map_read()?;
    let config_entry = gpu.uniform().entry(0)?.compute()?;
    let basis_entry = gpu.storage(false).entry(1)?.compute()?;
    let bind_layout = gpu.bind_layout(&[config_entry, basis_entry]).make()?;
    // let bind = gpu
    //     .bind()
    //     .layout(&bind_layout)
    //     .entry(0, &config)
    //     .entry(1, &basis)
    //     .make()?;
    //let wow = Hub::Tray(Tray::Base(Grc::new(bind_layout)));
    // let wow = Grc::new(bind_layout);
    let bind = gpu
        .binder()
        .layout(bind_layout.clone())
        .entry(0, config)
        .entry(1, basis.clone())
        .hub()?;
    let pipe_layout = gpu.pipe_layout(&[&bind_layout]).make()?;
    let pipe = shader.compute("main").layout(&pipe_layout).make()?;
    let mutator = gpu
        .dispatcher()
        .pipe(pipe)
        .bind(bind)
        .count(count)
        // .stage(basis, stage.clone())
        .hub()?;
    let out: Vec<f32> = gpu.reader(basis).stage(stage).mutator(mutator).hub()?.base().await?;
    assert_eq!(
        out,
        vec![
            0.0, 0.06666667, 0.13333334, 0.20000002, 0.26666668, 0.33333334, 0.40000004, 0.4666667,
            0.53333336, 0.6, 0.6666667, 0.73333335, 0.8000001, 0.86666673, 0.9333334, 1.0
        ]
    );
    Ok(())
}

// let mut encoder = gpu.encoder();
// encoder
//     .compute()
//     .pipe(&pipe)
//     .bind(0, &bind, &[])
//     .debug("compute collatz iterations")
//     .dispatch(9, 1, 1);
// encoder
//     .copy_buffer(&storage)
//     .destination(&stage)
//     .size(size)
//     .submit();

// #[rustfmt::skip]
// fn basic_f32() -> Vec<f32> {
//     vec![
//         1., 2., 3.,
//         4., 5., 6.,
//         7., 8., 9.,
//     ].into()
// }
