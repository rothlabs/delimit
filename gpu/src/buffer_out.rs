use super::*;
use web_sys::WebGlBuffer;

#[derive(Debug)]
pub struct BufferOut {
    gl: WGLRC,
    object: WebGlBuffer,
    target: u32,
    count: Hub<i32>,
}

impl BufferOut {
    pub fn make(gl: &WGLRC, target: u32, count: Hub<i32>) -> Result<Node<BufferOut>> {
        let object = gl
            .create_buffer()
            .ok_or(anyhow!("failed to create buffer"))?;
        Node::make(|_| {
            let buffer = Self {
                gl: gl.clone(),
                object,
                target,
                count,
            };
            Ok(buffer)
        })
    }
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.object));
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }
    pub fn bind_base(&self) {
        self.gl.bind_buffer_base(self.target, 0, Some(&self.object));
    }
    fn vec_f32(&self, array: &mut Vec<f32>) -> Result<()> {
        let view = unsafe {
            Float32Array::view(array.as_mut_slice())
        };
        self.gl.get_buffer_sub_data_with_i32_and_array_buffer_view(WGLRC::TRANSFORM_FEEDBACK_BUFFER, 0, &view);
        Ok(())
    }
}

impl Solve for BufferOut {
    type Base = Vf32;
    fn solve(&self, _: Task) -> Result<Gain<Self::Base>> {
        self.bind();
        // self.gl.buffer_data_with_i32(WGLRC::ARRAY_BUFFER, size, usage)
        self.unbind();
        solve_ok()
    }
}
