use super::*;
use web_sys::WebGlTransformFeedback;

/// Transform Feedback Object
/// Manage transform-feedback state on the GPU
#[derive(Builder, Clone, Debug)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"))]
pub struct Tfo {
    gl: WGLRC,
    object: WebGlTransformFeedback,
    buffers: Vec<Buffer>,
}

impl TfoBuilder {
    pub fn make(self) -> Result<Tfo> {
        let tfo = self.build()?;
        tfo.bind();
        for (i, buffer) in tfo.buffers.iter().enumerate() {
            tfo.gl.bind_buffer_base(WGLRC::TRANSFORM_FEEDBACK_BUFFER, i as u32, Some(&buffer.object));
        }
        tfo.unbind();
        Ok(tfo)
    }
}

impl Tfo {
    pub fn bind(&self) {
        self.gl.bind_transform_feedback(WGLRC::TRANSFORM_FEEDBACK, Some(&self.object));
    }
    pub fn unbind(&self) {
        self.gl.bind_transform_feedback(WGLRC::TRANSFORM_FEEDBACK, None);
    }
}

// impl Act for Tfo {
//     fn act(&self) -> Result<()> {
//         self.bind();
//         //self.buffer.read(|unit| unit.bind_base())?;
//         self.unbind();
//         Ok(())
//     }
// }
