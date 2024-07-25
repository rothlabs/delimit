use super::*;

pub type Result = std::result::Result<Agent<VertexAttribute>, VertexAttributeBuilderError>;

/// Tell the GPU how to read from a buffer
#[derive(Builder)]
#[builder(setter(into))]
pub struct VertexAttribute {
    gl: WGLRC,
    buffer: Agent<Buffer<f32>>,
    #[builder(default)]
    index: Value<u32>,
    #[builder(default)]
    size: Value<i32>,
    #[builder(default)]
    stride: Value<i32>,
    #[builder(default)]
    offset: Value<i32>,
}

impl VertexAttributeBuilder {
    pub fn link(&self) -> Result {
        let mut attrib = self.build()?;
        Ok(Agent::make(|back| {
            attrib.buffer = attrib.buffer.backed(back);
            attrib
        }))
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

// impl VertexAttributeBuilder {
//     pub fn link(&self) -> Result {
//         let mut attrib = self.build()?;
//         Ok(Agent::make(|back| {
//             attrib.buffer = attrib.buffer.backed(back);
//             attrib
//         }))

// impl VertexAttributeBuilder {
//     pub fn link(&self) -> Agent<VertexAttribute> {
//         let bldr = self.clone();
//         if let Ok(att) = self.build() {
//             Agent::make(|back|
//         }
//         Agent::make(|back|
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
//     pub fn link(wglrc: &WGLRC, buffer: &Agent<Buffer<f32>>) -> Agent<VertexAttribute> {
//         Agent::make(|back| Self {
//             gl: wglrc.clone(),
//             buffer: buffer.backed(back),
//             index: Value::default(),
//             size: Value::default(),
//             stride: Value::default(),
//             offset: Value::default(),
//         })
//         //agent.write(|pack| write(&mut pack.unit));
//         //agent
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
