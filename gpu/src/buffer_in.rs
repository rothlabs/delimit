use gloo_timers::callback::Timeout;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, WebGlSync};

use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct BufferIn {
    buffer: Buffer,
    size: Hub<i32>,
    draw: Node<DrawArrays>,
}

impl BufferInBuilder {
    pub fn make(self) -> Result<Hub<Vf32>> {
        self.build()?.hub()
    }
}

impl Adapt for BufferIn {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.size.deal("size", deal)?;
        Ok(())
    }
}

impl Solve for BufferIn {
    type Base = Vf32;
    async fn solve(&self) -> Result<Hub<Vf32>> {
        self.draw.act().await?;
        let sync = self
            .buffer
            .gl
            .fence_sync(WGLRC::SYNC_GPU_COMMANDS_COMPLETE, 0)
            .ok_or(anyhow!("make fenc sync failed"))?;
        let promise =
            Promise::new(&mut |resolve, _| poll(self.buffer.gl.clone(), sync.clone(), resolve));
        if let Err(err) = JsFuture::from(promise).await {
            Err(anyhow!("JsFuture failed {:?}", err))?
        }
        self.buffer.bind();
        let mut array = vec![0.; self.size.base().await? as usize];
        let view = unsafe { Float32Array::view(array.as_mut_slice()) };
        self.buffer
            .gl
            .get_buffer_sub_data_with_i32_and_array_buffer_view(WGLRC::ARRAY_BUFFER, 0, &view);
        self.buffer.unbind();
        Ok(array.leaf().hub())
    }
}

fn poll(gl: WGLRC, sync: WebGlSync, resolve: Function) {
    let status = gl.client_wait_sync_with_u32(&sync, WGLRC::SYNC_FLUSH_COMMANDS_BIT, 0);
    if status == WGLRC::TIMEOUT_EXPIRED {
        Timeout::new(1, || poll(gl, sync, resolve)).forget();
    } else if status == WGLRC::WAIT_FAILED {
        panic!("WGLRC::WAIT_FAILED");
    } else {
        let window = window().unwrap();
        if let Err(err) = window.set_timeout_with_callback(&resolve) {
            panic!("set_timeout_with_callback: {:?}", err)
        }
    }
}

impl Reckon for BufferIn {}

// impl BufferIn {
//     fn poll(&self) -> Result<()> {
//         let sync = self.gl.fence_sync(WGLRC::SYNC_GPU_COMMANDS_COMPLETE, 0).ok_or(anyhow!("make fenc sync failed"))?;
//         let status = self.gl.client_wait_sync_with_u32(&sync, WGLRC::SYNC_FLUSH_COMMANDS_BIT, 100);
//         if status == WGLRC::TIMEOUT_EXPIRED {
//             Timeout::new(1, || {self.poll();});
//             Ok(())
//         } else if status == WGLRC::WAIT_FAILED {
//             panic!("crap");
//         } else {
//             panic!("done");
//         }
//     }
// }

// impl BufferIn {
//     pub fn bind(&self) {
//         self.gl.bind_buffer(self.target, Some(&self.object));
//     }
//     pub fn unbind(&self) {
//         self.gl.bind_buffer(self.target, None);
//     }
//     pub fn bind_base(&self) {
//         self.gl.bind_buffer_base(self.target, 0, Some(&self.object));
//     }
// }

// pub fn make(gl: &WGLRC, target: u32, count: Hub<i32>) -> Result<Node<BufferOut>> {
//     let object = gl
//         .create_buffer()
//         .ok_or(anyhow!("failed to create buffer"))?;
//     Node::make(|_| {
//         let buffer = Self {
//             gl: gl.clone(),
//             object,
//             target,
//             count,
//         };
//         Ok(buffer)
//     })
// }
