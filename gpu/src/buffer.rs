use super::*;
use web_sys::WebGlBuffer;

pub type Result = std::result::Result<Node<Buffer>, graph::Error>;

#[derive(Debug)]
pub struct Buffer {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Apex,
}

impl Buffer {
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer));
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }
}

impl Buffer {
    pub fn link(gl: &WGLRC, target: u32, array: &Apex) -> Result {
        let buffer = gl.create_buffer().ok_or(anyhow!("failed to create buffer"))?;
        let link = Node::make(|back| Self {
            gl: gl.clone(),
            buffer,
            target,
            array: array.backed(back),
        });
        link.solve(Task::Main)?;
        Ok(link)
    }
}

impl Solve for Buffer {
    fn solve(&self, _: Task) -> solve::Result {
        self.bind();
        self.array.read(|array| 
            unsafe {
                match array? {
                    Tray::Vu16(array) => 
                    self.gl.buffer_data_with_array_buffer_view(
                        self.target,
                        &Uint16Array::view(array.as_slice()),
                        WGLRC::STATIC_DRAW,
                    ),
                    Tray::Vf32(array) => 
                    self.gl.buffer_data_with_array_buffer_view(
                        self.target,
                        &Float32Array::view(array.as_slice()),
                        WGLRC::STATIC_DRAW,
                    ),
                    _ => () //return Err(wrong_tray("Vu16", array.clone()))?
                };
                Ok(())
            }
        )?;
        self.unbind();
        Ok(Gain::None)
    }
}