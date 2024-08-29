use super::*;
use web_sys::WebGlBuffer;

// pub type Result = std::result::Result<Node<Buffer>, graph::Error>;

#[derive(Debug)]
pub struct Buffer {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Apex,
}

impl Buffer {
    pub fn link(gl: &WGLRC, target: u32, array: &Apex) -> Result<Node<Buffer>> {
        let buffer = gl
            .create_buffer()
            .ok_or(anyhow!("failed to create buffer"))?;
        let link = Node::make(|back| Self {
            gl: gl.clone(),
            buffer,
            target,
            array: array.backed(back),
        });
        link.solve(Task::Main)?;
        Ok(link)
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

impl Solve for Buffer {
    fn solve(&self, _: Task) -> solve::Result {
        self.bind();
        self.array.read(|tray| self.set(tray))??;
        self.unbind();
        solve_ok()
    }
}
