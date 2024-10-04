//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use dom::*;
use gpu::*;
use graph::*;
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

fn vertex_buffer(gpu: &Gpu, size: u64) -> gpu::Result<Buffer> {
    gpu.buffer().size(size).usage(BufferUsages::VERTEX).make()
}

fn storage_buffer(gpu: &Gpu, size: u64) -> gpu::Result<Buffer> {
    gpu.buffer()
        .size(size)
        .usage(BufferUsages::STORAGE | BufferUsages::COPY_DST | BufferUsages::COPY_SRC)
        .make()
}

// #[rustfmt::skip]
// fn basic_f32() -> Vf32 {
//     vec![
//         1., 2., 3.,
//         4., 5., 6.,
//         7., 8., 9.,
//     ].into()
// }

#[rustfmt::skip]
fn basic_u32() -> Vec<u32> {
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
    storage.writer().data(basic_u32()).make()?.act().await?;
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
    console_log!("result: {:?}", out);
    Ok(())
}

// let reader: Node<BufferReader<u32>> = stage.reader().node()?;
// let out = reader.solve().await?.base().await?;

// #[wasm_bindgen_test]
// async fn make_buffer_writer() -> dom::Result<()> {
//     let gpu = gpu().await?;
//     storage_buffer_with_writer(&gpu)?;
//     Ok(())
// }

// fn buffer_writer(buffer: Buffer, data ) -> dom::Result<Node<BufferWriter>> {
//     #[rustfmt::skip]
//     let data: Vec<f32> = vec![
//         0., 0., 0.,
//         1., 0., 0.,
//         0., 1., 0.,
//     ];
//     let writer = buffer.writer().data(data).node()?;
//     Ok(writer)
// }

// let mut encoder = gpu.encoder();
//     {
//         let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
//             label: None,
//             timestamp_writes: None,
//         });
//         cpass.set_pipeline(&pipe);
//         cpass.set_bind_group(0, &bind_group, &[]);
//         cpass.insert_debug_marker("compute collatz iterations");
//         cpass.dispatch_workgroups(9, 1, 1);
//     }
//     encoder.copy_buffer_to_buffer(&storage, 0, &stage, 0, 36);
//     gpu.submit(encoder);
