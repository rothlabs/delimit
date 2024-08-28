use super::*;

pub type Result = std::result::Result<Node<VertexAttribute>, VertexAttributeBuilderError>;

/// Tell the GPU how to read from a buffer
#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct VertexAttribute {
    gl: WGLRC,
    buffer: Node<Buffer>, // f32
    /// Location in vertex shader. `layout(location = index)`
    #[builder(default)]
    index: Apex, // u32
    /// Number of components per value
    #[builder(default)]
    size: Apex, //i32
    /// Number of bytes between values
    #[builder(default)]
    stride: Apex, // i32
    /// Byte offset of first value
    #[builder(default)]
    offset: Apex, // i32
}

impl VertexAttributeBuilder {
    pub fn link(&self) -> Result {
        let mut attrib = self.build()?;
        Ok(Node::make(|back| {
            attrib.buffer = attrib.buffer.backed(back);
            attrib.index = attrib.index.backed(back);
            attrib.size = attrib.size.backed(back);
            attrib.stride = attrib.stride.backed(back);
            attrib.offset = attrib.offset.backed(back);
            attrib
        }))
    }
}

impl VertexAttribute {
    fn set(&self, buffer: &Buffer, index: u32) -> GraphResult<()> {
        buffer.bind();
        self.gl.vertex_attrib_pointer_with_i32(
            index,
            self.size.i32().unwrap_or_default(),
            WGLRC::FLOAT,
            false,
            self.stride.i32().unwrap_or_default(),
            self.offset.i32().unwrap_or_default(),
        );
        self.gl.enable_vertex_attrib_array(index);
        buffer.unbind();
        Ok(())
    }
}

impl Solve for VertexAttribute {
    fn solve(&self, _: Task) -> solve::Result {
        let index = self.index.u32().unwrap_or_default();
        self.buffer.solve(Task::Main)?;
        self.buffer.read(|buffer| self.set(buffer?, index))?;
        Ok(Gain::None)
    }
}

// impl React for VertexAttribute {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act();
//         Ok(())
//     }
// }

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
