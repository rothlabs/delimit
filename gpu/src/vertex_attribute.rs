use super::*;

/// Tell the GPU how to read from a buffer
#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into))]
pub struct VertexAttribute {
    gl: WGLRC,
    buffer: BufferBinder,
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
    pub fn make(&self) -> Result<Node<VertexAttribute>> {
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
    fn act(&self) -> Result<()> {
        self.buffer.bind();
        let index = self.index.base().unwrap_or_default();
        self.gl.vertex_attrib_pointer_with_i32(
            index,
            self.size.base().unwrap_or_default(),
            WGLRC::FLOAT,
            false,
            self.stride.base().unwrap_or_default(),
            self.offset.base().unwrap_or_default(),
        );
        self.gl.enable_vertex_attrib_array(index);
        self.buffer.unbind();
        Ok(())
    }
}

// self.gl.vertex_attrib_pointer_with_f64(indx, size, type_, normalized, stride, offset)

// impl VertexAttributeBuilder {
//     pub fn link(&self) -> Result {
//         let mut attrib = self.build()?;
//         Ok(Node::make(|back| {
//             attrib.buffer = attrib.buffer.backed(back);
//             attrib
//         }))

// impl VertexAttributeBuilder {
//     pub fn link(&self) -> Node<VertexAttribute> {
//         let bldr = self.clone();
//         if let Ok(att) = self.build() {
//             Node::make(|back|
//         }
//         Node::make(|back|
//             VertexAttribute {
//                 gl: bldr.gl.unwrap(),
//                 buffer: bldr.buffer.unwrap().backed(back),
//                 index: bldr.index.unwrap_or(Value::default()),
//                 size: bldr.size.unwrap_or(Value::default()),
//                 stride: bldr.stride.unwrap_or(Value::default()),
//                 offset: bldr.offset.unwrap_or(Value::default()),
//             }
//         )
//     }
// }

// impl VertexAttribute {
//     // <F: FnOnce(&mut Self)> , write: F
//     pub fn link(wglrc: &WGLRC, buffer: &Node<Buffer<f32>>) -> Node<VertexAttribute> {
//         Node::make(|back| Self {
//             gl: wglrc.clone(),
//             buffer: buffer.backed(back),
//             index: Value::default(),
//             size: Value::default(),
//             stride: Value::default(),
//             offset: Value::default(),
//         })
//         //node.write(|pack| write(&mut pack.unit));
//         //node
//     }
//     pub fn index(&mut self, index: impl Into<Value<u32>>) -> &mut Self {
//         self.index = index.into();
//         self
//     }
//     pub fn size(&mut self, size: impl Into<Value<i32>>) -> &mut Self {
//         self.size = size.into();
//         self
//     }
//     pub fn stride(&mut self, stride: impl Into<Value<i32>>) -> &mut Self {
//         self.stride = stride.into();
//         self
//     }
//     pub fn offset(&mut self, offset: impl Into<Value<i32>>) -> &mut Self {
//         self.offset = offset.into();
//         self
//     }
// }
