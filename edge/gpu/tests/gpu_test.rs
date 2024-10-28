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

async fn gpu() -> dom::Result<Gpu> {
    let canvas = body()?.element("canvas")?.canvas()?;
    canvas.gpu().await
}

async fn gpu_with_canvas<'a>() -> dom::Result<Gpu> {
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

#[rustfmt::skip]
fn triangle_data() -> Vec<f32> {
    vec![
        // pos             color
        -0.3_f32, -0.0,    1., 0., 0., 0.,
         0.,       0.6,    0., 1., 0., 0.,
         0.3,     -0.0,    0., 0., 1., 0.,
         0.,      -0.6,    0., 1., 0., 0.,
    ]
}

#[rustfmt::skip]
fn instance_data() -> Vec<f32> {
    vec![
        // pos         
        -0.7_f32, -0.7,
         0.,       0.7,
         0.7,     -0.7,
         1.,       1.,
    ]
}

#[rustfmt::skip]
fn index_data() -> Vec<u16> {
    vec![0, 1, 2,   0, 2, 3]
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
    let targets = gpu.surface.targets();
    let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
    let vertex = shader.vertex("vs_main").make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    // render:
    let view = gpu.surface.view();
    gpu.command()
        .texture_view(view)
        .render(pipe)
        .draw(0..3, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}

const BASIC_SHADER: ShaderModuleDescriptor = include_wgsl!("../src/shader/basic.wgsl");
const BASIC_INSTANCE_SHADER: ShaderModuleDescriptor =
    include_wgsl!("../src/shader/basic_instance.wgsl");

#[wasm_bindgen_test]
async fn draw_lines() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let targets = gpu.surface.targets();
    let shader = gpu.shader(BASIC_SHADER);
    let prim = gpu.lines().make()?;
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let buffers = vec![gpu.vertex_layout(24).attributes(&attribs).make()?];
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu
        .render_pipe(vertex)
        .fragment(fragment)
        .primitive(prim)
        .make()?;
    // TODO: make buffer_vertex return Hub<Grc<Buffer>>
    let buffer = gpu.vertex_buffer(&line_data());
    let view = gpu.surface.view();
    gpu.command()
        .texture_view(view)
        .render(pipe)
        .vertex(0, buffer)
        .draw(0..4, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_msaa_lines() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let targets = gpu.surface.targets();
    let shader = gpu.shader(BASIC_SHADER);
    let prim = gpu.lines().make()?;
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let buffers = vec![gpu.vertex_layout(24).attributes(&attribs).make()?];
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let multi = gpu.multisample(4).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu
        .render_pipe(vertex)
        .fragment(fragment)
        .primitive(prim)
        .multisample(multi)
        .make()?;
    let buffer = gpu.vertex_buffer(&line_data());
    let view = gpu.surface.view();
    let texture_view = gpu.surface.texture().sample_count(4).view()?;
    gpu.command()
        .texture_view(texture_view)
        .resolve_target(view)
        .render(pipe)
        .vertex(0, buffer)
        .draw(0..4, 0..1)
        .hub()?
        .base()
        .await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_triangle_instances() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let targets = gpu.surface.targets();
    let shader = gpu.shader(BASIC_INSTANCE_SHADER);
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let model = gpu.vertex_layout(24).attributes(&attribs).make()?;
    let attribs = vertex_attr_array![2 => Float32x2];
    let instance = gpu.vertex_layout(8).attributes(&attribs).instance()?;
    let buffers = vec![model, instance];
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    let view = gpu.surface.view();
    let model = gpu.vertex_buffer(&triangle_data());
    let instance = gpu.vertex_buffer(&instance_data());
    gpu.command()
        .texture_view(view)
        .render(pipe)
        .vertex(0, model)
        .vertex(1, instance)
        .draw(0..3, 0..4)
        .hub()?
        .base()
        .await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn draw_triangle_indexed_instance() -> dom::Result<()> {
    let gpu = gpu_with_canvas().await?;
    let targets = gpu.surface.targets();
    let shader = gpu.shader(BASIC_INSTANCE_SHADER);
    let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
    let model = gpu.vertex_layout(24).attributes(&attribs).make()?;
    let attribs = vertex_attr_array![2 => Float32x2];
    let instance = gpu.vertex_layout(8).attributes(&attribs).instance()?;
    let buffers = vec![model, instance];
    let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
    let fragment = shader.fragment("fs_main").targets(targets).make()?;
    let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
    let view = gpu.surface.view();
    let model = gpu.vertex_buffer(&triangle_data());
    let instance = gpu.vertex_buffer(&instance_data());
    let index = gpu.index_buffer(&index_data());
    gpu.command()
        .texture_view(view)
        .render(pipe)
        .vertex(0, model)
        .vertex(1, instance)
        .index(index)
        .draw_indexed(0..6, 0, 0..4)
        .hub()?
        .base()
        .await?;
    Ok(())
}

#[wasm_bindgen_test]
async fn compute_collatz_iterations() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(include_wgsl!("collatz.wgsl"));
    let pipe = shader.compute("main").make()?;
    let size = 36;
    let storage = gpu.buffer(size).storage()?;
    let bind = gpu
        .bind()
        .pipe(pipe.clone())
        .entry(0, storage.clone())
        .hub()?;
    gpu.writer(storage.clone())
        .data(basic_vec_u32())
        .hub()?
        .base()
        .await?;
    let collatz = gpu
        .command()
        .compute(pipe)
        .bind(0, bind)
        .dispatch(9)
        .hub()?;
    let out = gpu
        .reader::<u32>(storage)
        .root(collatz)
        .staged()?
        .base()
        .await?;
    assert_eq!(out, vec![0, 1, 7, 2, 5, 8, 16, 3, 19]);
    Ok(())
}

#[wasm_bindgen_test]
async fn index_fraction() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(include_wgsl!("index.wgsl"));
    let count = 16;
    let size = 4 * count as u64;
    let rig = gpu.uniform().field(count).make()?;
    let basis = gpu.buffer(size).storage()?;
    let config_entry = gpu.bind_uniform().entry(0)?.compute()?;
    let basis_entry = gpu.bind_storage(false).entry(1)?.compute()?;
    let bind_layout = gpu.bind_layout(&[config_entry, basis_entry]).make()?;
    let bind = gpu
        .bind()
        .layout(bind_layout.clone())
        .entry(0, rig.buffer)
        .entry(1, basis.clone())
        .hub()?;
    let pipe_layout = gpu.pipe_layout(&[&bind_layout]).make()?;
    let pipe = shader.compute("main").layout(&pipe_layout).make()?;
    let index_compute = gpu
        .command()
        .root(rig.root)
        .compute(pipe)
        .bind(0, bind)
        .dispatch(count)
        .hub()?;
    let out: Vec<f32> = gpu
        .reader(basis)
        .root(index_compute)
        .staged()?
        .base()
        .await?;
    assert_eq!(
        out,
        vec![
            0.0, 0.06666667, 0.13333334, 0.20000002, 0.26666668, 0.33333334, 0.40000004, 0.4666667,
            0.53333336, 0.6, 0.6666667, 0.73333335, 0.8000001, 0.86666673, 0.9333334, 1.0
        ]
    );
    Ok(())
}


// #[wasm_bindgen_test]
// async fn draw_triangle_instances() -> dom::Result<()> {
//     let gpu = gpu_with_canvas().await?;
//     let targets = gpu.surface.targets();
//     let shader = gpu.shader(BASIC_INSTANCE_SHADER);
//     let attribs = vertex_attr_array![0 => Float32x2, 1 => Float32x4];
//     let model = gpu.vertex_layout(24).attributes(&attribs).make()?;
//     let attribs = vertex_attr_array![2 => Float32x2];
//     let instance = gpu.vertex_layout(8).attributes(&attribs).instance()?;
//     let buffers = vec![model, instance];
//     let vertex = shader.vertex("vs_main").buffers(&buffers).make()?;
//     let fragment = shader.fragment("fs_main").targets(targets).make()?;
//     let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
//     let view = gpu.surface.view();
//     let model = gpu.vertex_buffer(&triangle_data());
//     let instance = gpu.vertex_buffer(&instance_data());
//     gpu.command()
//         .texture_view(view)
//         .render(pipe)
//         .vertex(0, model)
//         .vertex(1, instance)
//         .draw(0..3, 0..4)
//         .hub()?
//         .base()
//         .await?;
//     Ok(())
// }








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
