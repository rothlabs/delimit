use super::*;
use web_sys::WebGlVertexArrayObject;

pub type Attributes = Vec<Node<VertexAttribute>>;

/// Vertex Array Object
/// Stores attribute settings and element array buffer target
#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into, strip_option))]
pub struct Vao {
    gl: WGLRC,
    object: WebGlVertexArrayObject,
    attributes: Attributes,
    /// for ELEMENT_ARRAY_BUFFER only (ARRAY_BUFFER has no effect)
    index_buffer: Option<Node<Buffer>>, // u16
}

impl VaoBuilder {
    pub fn make(&self) -> Result<Node<Vao>> {
        let mut vao = self.build()?;
        Node::make(|back| {
            vao.attributes = vao.attributes.backed(back)?;
            if let Some(index_buffer) = vao.index_buffer {
                vao.index_buffer = Some(index_buffer.backed(back)?);
            }
            Ok(vao)
        })
    }
}

impl Vao {
    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.object));
    }
    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }
}

impl Act for Vao {
    fn act(&self) -> Result<()> {
        self.bind();
        for attribute in &self.attributes {
            attribute.act()?;
        }
        if let Some(buffer) = &self.index_buffer {
            buffer.act()?;
            buffer.read(|unit| unit.bind())?;
        }
        self.unbind();
        Ok(())
    }
}

// impl React for Vao {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act();
//         Ok(())
//     }
// }

// pub fn link(gl: &WGLRC, attributes: &Attributes) -> Result {
//     let object = gl
//         .create_vertex_array()
//         .ok_or("failed to create vertex array object")?;
//     let link = Node::make(|back| Self {
//         gl: gl.clone(),
//         object,
//         attributes: attributes.backed(back),
//         index_buffer: None,
//     });
//     link.act();
//     Ok(link)
// }
// pub fn index_buffer(&mut self, buffer: Node<Buffer<u16>>) -> &mut Self {
//     self.index_buffer = Some(buffer);
//     self
// }

// impl Vao {
//     pub fn link(gl: &WGLRC, attributes: &Attributes) -> Result {
//         let object = gl
//             .create_vertex_array()
//             .ok_or("failed to create vertex array object")?;
//         let link = Node::make(|back| Self {
//             gl: gl.clone(),
//             object,
//             attributes: attributes.backed(back),
//             index_buffer: None,
//         });
//         link.act();
//         Ok(link)
//     }
//     pub fn index_buffer(&mut self, buffer: Node<Buffer<u16>>) -> &mut Self {
//         self.index_buffer = Some(buffer);
//         self
//     }
//     pub fn bind(&self) {
//         self.gl.bind_vertex_array(Some(&self.object));
//     }
//     pub fn unbind(&self) {
//         self.gl.bind_vertex_array(None);
//     }
// }
