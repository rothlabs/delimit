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
// fn basic_data() -> Vec<f32> {
//     vec![
//         1., 2., 3.,
//         4., 5., 6.,
//         7., 8., 9.,
//     ]
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
async fn make_compute_pipeline() -> dom::Result<()> {
    let gpu = gpu().await?;
    let shader = gpu.shader(wgpu::include_wgsl!("compute.wgsl"));
    let pipe = gpu.compute().shader(&shader).entry("main").make()?;
    let storage = storage_buffer(&gpu, 36)?;
    let stage = gpu
        .buffer()
        .label("stage")
        .size(36)
        .map_read()
        .make()?;
    let writer = storage.writer().data(basic_u32()).make()?;
    writer.act().await?;
    let bind_group = gpu.bind_group().pipe(&pipe).entry(0, &storage).make()?;
    let mut encoder = gpu.encoder();
    {
        let mut cpass = encoder.compute();
        cpass.set_pipeline(&pipe);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute collatz iterations");
        cpass.dispatch_workgroups(9, 1, 1);
    }
    encoder.copy_buffer(&storage).to_buffer(&stage).size(36).submit();
    let buffer_slice = stage.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    // gpu.device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    if let Ok(Ok(())) = receiver.recv_async().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result: Vec<u32> = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        stage.unmap(); // Unmaps buffer from memory
                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                //   delete myPointer;
                                //   myPointer = NULL;
                                // It effectively frees the memory

        // Returns data from buffer
        // Some(result)
        console_log!("result: {:?}", result);
        Ok(())
    } else {
        panic!("failed to run compute on gpu!")
    }
}

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