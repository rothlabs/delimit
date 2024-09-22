use super::*;

/// Tell the GPU how to read from a buffer
#[derive(Builder, Debug)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"), setter(into))]
pub struct VertexAttribute {
    // gl: WGLRC,
    buffer: Buffer,
    #[builder(default = "WGLRC::FLOAT")]
    type_: u32,
    /// Location in vertex shader. `layout(location = index)`
    #[builder(default)]
    index: Hub<u32>,
    /// Number of components per value
    #[builder(default)]
    size: Hub<i32>,
    /// Number of bytes between values
    #[builder(default)]
    stride: Hub<i32>,
    /// Byte offset of first value
    #[builder(default)]
    offset: Hub<i32>,
    #[builder(default)]
    divisor: Hub<i32>,
}

impl VertexAttributeBuilder {
    pub fn make(self) -> Result<Node<VertexAttribute>> {
        self.build()?.node()
        // let mut attrib = self.build()?;
        // Node::make(|back| {
        //     attrib.index = attrib.index.backed(back)?;
        //     attrib.size = attrib.size.backed(back)?;
        //     attrib.stride = attrib.stride.backed(back)?;
        //     attrib.offset = attrib.offset.backed(back)?;
        //     attrib.divisor = attrib.divisor.backed(back)?;
        //     Ok(attrib)
        // })
    }
}

impl Act for VertexAttribute {
    fn back(&mut self, back: &Back) -> Result<()> {
        self.index = self.index.backed(back)?;
        self.size = self.size.backed(back)?;
        self.stride = self.stride.backed(back)?;
        self.offset = self.offset.backed(back)?;
        self.divisor = self.divisor.backed(back)?;
        Ok(())
    }
    async fn act(&self) -> Result<()> {
        self.buffer.bind();
        let index = self.index.base().await.unwrap_or_default();
        self.buffer.gl.vertex_attrib_pointer_with_i32(
            index,
            self.size.base().await.unwrap_or_default(),
            self.type_,
            false,
            self.stride.base().await.unwrap_or_default(),
            self.offset.base().await.unwrap_or_default(),
        );
        let divisor = self.divisor.base().await.unwrap_or_default();
        if divisor > 0 {
            self.buffer.gl.vertex_attrib_divisor(index, divisor as u32);
        }
        self.buffer.gl.enable_vertex_attrib_array(index);
        self.buffer.unbind();
        Ok(())
    }
}

impl Reckon for VertexAttribute {}
