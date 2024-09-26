use super::*;

#[derive(Builder, Clone, Debug)]
#[builder(build_fn(error = "graph::Error"), pattern = "owned")]
pub struct Buffer {
    pub gl: WGLRC,
    pub object: WebGlBuffer,
    #[builder(default = "WGLRC::ARRAY_BUFFER")]
    pub target: u32,
}

impl Buffer {
    pub fn index(mut self) -> Self {
        self.target = WGLRC::ELEMENT_ARRAY_BUFFER;
        self
    }
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.object))
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None)
    }
    pub fn writer(&self) -> BuffererBuilder {
        BuffererBuilder::default().buffer(self)
    }
    pub fn reader(&self) -> BufferInBuilder {
        BufferInBuilder::default().buffer(self)
    }
    pub fn attribute(&self) -> VertexAttributeBuilder {
        VertexAttributeBuilder::default().buffer(self)
    }
}

impl From<&Buffer> for Buffer {
    fn from(value: &Buffer) -> Self {
        value.clone()
    }
}
