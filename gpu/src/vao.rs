use super::*;
use web_sys::WebGlVertexArrayObject;

/// Vertex Array Object
/// Stores attribute settings and element array buffer target
#[attr_alias::eval]
#[derive(Builder, Debug, Make!)]
#[attr_alias(build)]
pub struct Vao {
    gl: WGLRC,
    object: WebGlVertexArrayObject,
    #[builder(setter(each(name = "attribute")))]
    attributes: Vec<Node<VertexAttribute>>,
    /// for ELEMENT_ARRAY_BUFFER only (ARRAY_BUFFER has no effect)
    #[builder(default)]
    index: Option<Buffer>,
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
    async fn act(&self) -> Result<()> {
        self.bind();
        for attribute in &self.attributes {
            attribute.act().await?;
        }
        if let Some(buffer) = &self.index {
            buffer.bind();
        }
        self.unbind();
        Ok(())
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.attributes.back(back)
    }
}
