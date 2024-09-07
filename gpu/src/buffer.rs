use super::*;
use web_sys::WebGlBuffer;

#[derive(Debug)]
pub struct Buffer {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Hub,
}

impl Buffer {
    pub fn make(gl: &WGLRC, target: u32, array: &Hub) -> Result<Node<Buffer>> {
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
    fn set(&self, tray: &Tray) -> Result<()> {
        match tray {
            Tray::Vu16(array) => unsafe {
                self.gl.buffer_data_with_array_buffer_view(
                    self.target,
                    &Uint16Array::view(array.as_slice()),
                    WGLRC::STATIC_DRAW,
                )
            },
            Tray::Vf32(array) => unsafe {
                self.gl.buffer_data_with_array_buffer_view(
                    self.target,
                    &Float32Array::view(array.as_slice()),
                    WGLRC::STATIC_DRAW,
                )
            },
            tray => return Err(tray.wrong_variant("Vec<u16> or Vec<f32>"))?,
        };
        Ok(())
    }
}

impl Act for Buffer {
    fn act(&self) -> graph::Result<()> {
        self.bind();
        self.array.read(|tray| self.set(tray))??;
        self.unbind();
        Ok(())
    }
}

impl Adapt for Buffer {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}
