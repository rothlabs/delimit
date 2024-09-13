use super::*;

#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"))]
pub struct Buffer {
    gl: WGLRC,
    pub buffer: WebGlBuffer,
    #[builder(default = "WGLRC::ARRAY_BUFFER")]
    pub target: u32,
}

impl Buffer {
    pub fn index(mut self) -> Self {
        self.target = WGLRC::ELEMENT_ARRAY_BUFFER;
        self
    }
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer))
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None)
    }
}

impl From<&Buffer> for Buffer {
    fn from(value: &Buffer) -> Self {
        value.clone()
    }
}