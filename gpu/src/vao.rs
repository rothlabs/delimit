use super::*;

pub type Result = std::result::Result<Agent<Vao>, String>;
pub type Attributes = Vec<Agent<VertexAttribute>>;

/// Vertex Array Object
/// Stores attribute settings and which element array buffer to use
pub struct Vao {
    gl: WGLRC,
    target: WebGlVertexArrayObject,
    attributes: Attributes,
    /// for ELEMENT_ARRAY_BUFFER only (ARRAY_BUFFER has no effect)
    element_buffer: Option<Agent<Buffer<u16>>>,
}

impl Vao {
    pub fn link(wglrc: &WGLRC, attributes: &Attributes) -> Result {
        let target = wglrc
            .create_vertex_array()
            .ok_or("failed to create vertex array object")?;
        let link = Agent::make(|back| Self {
            gl: wglrc.clone(),
            target,
            attributes: attributes.backed(back),
            element_buffer: None,
        });
        link.act();
        Ok(link)
    }
    pub fn element_buffer(&mut self, buffer: Agent<Buffer<u16>>) -> &mut Self {
        self.element_buffer = Some(buffer);
        self
    }
    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.target));
    }
    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }
}

impl Act for Vao {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.bind();
        for attribute in &self.attributes {
            attribute.act();
        }
        if let Some(buffer) = &self.element_buffer {
            buffer.act();
            buffer.read(|unit| unit.bind());
        }
        self.unbind();
    }
}

impl React for Vao {
    fn react(&self, _: &Meta) -> react::Result {
        self.act();
        Ok(())
    }
}
