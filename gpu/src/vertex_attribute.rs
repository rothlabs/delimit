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
        let size = self.size.load();
        let stride = self.stride.load();
        let offset = self.offset.load();
        self.buffer.act();
        self.buffer.read(|buffer| {
            buffer.bind();
            self.gl.enable_vertex_attrib_array(index);
            self.gl.vertex_attrib_pointer_with_i32(
                index,
                size,
                WGLRC::FLOAT,
                false,
                stride,
                offset,
            );
            buffer.unbind();
        });
    }
}

impl React for VertexAttribute {
    fn react(&self, _: &Meta) -> ReactResult {
        self.act();
        Ok(())
    }
}

// self.gl.vertex_attrib_pointer_with_f64(indx, size, type_, normalized, stride, offset)
