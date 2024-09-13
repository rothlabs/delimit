use super::*;

#[derive(Debug)]
pub struct Buffer {
    pub gl: WGLRC,
    pub object: WebGlBuffer,
    pub target: u32,
}

impl Buffer {
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.object))
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None)
    }
}

impl Act for Buffer {
    fn act(&self) -> Result<()> {
        Ok(())
    }
}