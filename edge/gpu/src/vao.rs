use super::*;
use web_sys::WebGlVertexArrayObject;

/// Vertex Array Object
/// Stores attribute settings and element array buffer target
#[derive(Clone, Debug)]
pub struct Vao {
    pub gl: WGLRC,
    pub object: WebGlVertexArrayObject,
}

impl Vao {
    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.object));
    }
    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }
    pub fn writer(&self) -> VaoWriterBuilder {
        VaoWriterBuilder::default().object(self)
    }
}

impl From<&Vao> for Vao {
    fn from(value: &Vao) -> Self {
        value.clone()
    }
}
