use super::*;

/// Tell the GPU how to read from a buffer
pub struct VertexAttribute {
    gl: WGLRC,
    buffer: Agent<Buffer<f32>>,
    index: Value<u32>,
    size: Value<i32>,
    stride: Value<i32>,
    offset: Value<i32>,
}

impl VertexAttribute {
    pub fn link(wglrc: &WGLRC, buffer: &Agent<Buffer<f32>>) -> Agent<VertexAttribute> {
        Agent::make(|back| Self {
            gl: wglrc.clone(),
            buffer: buffer.backed(back),
            index: Value::default(),
            size: Value::default(),
            stride: Value::default(),
            offset: Value::default(),
        })
    }
    pub fn index(&mut self, index: impl Into<Value<u32>>) -> &mut Self {
        self.index = index.into();
        self
    }
    pub fn size(&mut self, size: impl Into<Value<i32>>) -> &mut Self {
        self.size = size.into();
        self
    }
    pub fn stride(&mut self, stride: impl Into<Value<i32>>) -> &mut Self {
        self.stride = stride.into();
        self
    }
    pub fn offset(&mut self, offset: impl Into<Value<i32>>) -> &mut Self {
        self.offset = offset.into();
        self
    }
}

impl Act for VertexAttribute {
    type Load = ();
    fn act(&self) -> Self::Load {
        let index = self.index.load();
        self.buffer.act();
        self.buffer.read(|buffer| {
            buffer.bind();
            self.gl.vertex_attrib_pointer_with_i32(
                index,
                self.size.load(),
                WGLRC::FLOAT,
                false,
                self.stride.load(),
                self.offset.load(),
            );
            self.gl.enable_vertex_attrib_array(index);
            buffer.unbind();
        });
    }
}

impl React for VertexAttribute {
    fn react(&self, _: &Meta) -> react::Result {
        self.act();
        Ok(())
    }
}

// self.gl.vertex_attrib_pointer_with_f64(indx, size, type_, normalized, stride, offset)
