use super::*;

pub type Result = std::result::Result<Agent<Vao>, String>;
pub type Attributes = Vec<Agent<VertexAttribute>>;

/// Vertex Array Object
/// Stores attribute settings and which element array buffer to use 
pub struct Vao {
    gl: WGLRC,
    target: WebGlVertexArrayObject,
    attributes: Attributes,
    element_buffer: Option<Agent<Buffer>>,
}

impl Vao {
    pub fn link(wglrc: &WGLRC, attributes: &Attributes) -> Result {
        let target = wglrc.create_vertex_array().ok_or("failed to create vertex array object")?;
        let link = Agent::make(|back| Self { 
            gl: wglrc.clone(), 
            target, 
            attributes: attributes.backed(back),
            element_buffer: None,
        });
        link.act();
        Ok(link)
    }
}

impl Act for Vao {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.gl.bind_vertex_array(Some(&self.target));
        for attribute in &self.attributes {
            attribute.act();
        }
        if let Some(buffer) = &self.element_buffer {
            buffer.act();
        }
        self.gl.bind_vertex_array(None);
    }
}

