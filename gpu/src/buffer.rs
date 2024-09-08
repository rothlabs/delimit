use super::*;
use web_sys::WebGlBuffer;

#[derive(Debug)]
pub struct Buffer {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Apex,
}

impl Buffer {
    pub fn make(gl: &WGLRC, target: u32, array: &Apex) -> Result<Node<Buffer>> {
        let buffer = gl
            .create_buffer()
            .ok_or(anyhow!("failed to create buffer"))?;
        let node = Node::make(|back| {
            let buffer = Self {
                gl: gl.clone(),
                buffer,
                target,
                array: array.backed(back)?,
            };
            Ok(buffer)
        })?;
        node.act()?;
        Ok(node)
    }
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer));
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }
    fn vec_u16(&self, array: &Vec<u16>) -> Result<()> {
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.target,
                &Uint16Array::view(array.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        }
        Ok(())
    }
    fn vec_f32(&self, array: &Vec<f32>) -> Result<()> {
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.target,
                &Float32Array::view(array.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        }
        Ok(())
    }
}

impl Act for Buffer {
    fn act(&self) -> graph::Result<()> {
        self.bind();
        match &self.array {
            Apex::Vu16(array) => array.read(|array| self.vec_u16(array))?,
            Apex::Vf32(array) => array.read(|array| self.vec_f32(array))?,
            _ => Err(anyhow!("wrong apex"))?,
        }?;
        self.unbind();
        Ok(())
    }
}

impl Adapt for Buffer {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}
