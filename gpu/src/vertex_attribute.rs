use super::*;

/// Tell the GPU how to read from a buffer
#[derive(Builder, Debug)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"), setter(into))]
pub struct VertexAttribute {
    gl: WGLRC,
    buffer: Buffer,
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
}

impl VertexAttributeBuilder {
    pub fn make(self) -> Result<Node<VertexAttribute>> {
        let mut attrib = self.build()?;
        Node::make(|back| {
            attrib.index = attrib.index.backed(back)?;
            attrib.size = attrib.size.backed(back)?;
            attrib.stride = attrib.stride.backed(back)?;
            attrib.offset = attrib.offset.backed(back)?;
            Ok(attrib)
        })
    }
}

impl Act for VertexAttribute {
    async fn act(&self) -> Result<()> {
        self.buffer.bind();
        let index = self.index.base().await.unwrap_or_default();
        self.gl.vertex_attrib_pointer_with_i32(
            index,
            self.size.base().await.unwrap_or_default(),
            WGLRC::FLOAT,
            false,
            self.stride.base().await.unwrap_or_default(),
            self.offset.base().await.unwrap_or_default(),
        );
        self.gl.enable_vertex_attrib_array(index);
        self.buffer.unbind();
        Ok(())
    }
}

impl Reckon for VertexAttribute {
    
}