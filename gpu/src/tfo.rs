use super::*;
use web_sys::WebGlTransformFeedback;

/// Transform Feedback Object
/// Manage transform-feedback state on the GPU
#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
pub struct Tfo {
    gl: WGLRC,
    object: WebGlTransformFeedback,
    buffer: Node<Buffer>
}

impl TfoBuilder {
    pub fn make(&self) -> Result<Node<Tfo>> {
        let mut tfo = self.build()?;
        Node::make(|back| {
            tfo.buffer = tfo.buffer.backed(back)?;
            Ok(tfo)
        })
    }
}

impl Tfo {
    pub fn builder() -> TfoBuilder {
        TfoBuilder::default()
    }
    pub fn bind(&self) {
        self.gl.bind_transform_feedback(0, Some(&self.object));
    }
    pub fn unbind(&self) {
        self.gl.bind_transform_feedback(0, None);
    }
}

impl Act for Tfo {
    fn act(&self) -> Result<()> {
        self.bind();
        self.buffer.read(|unit| unit.bind_base())?;
        self.unbind();
        Ok(())
    }
}
