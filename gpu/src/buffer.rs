use super::*;
use web_sys::WebGlBuffer;

pub type Result = std::result::Result<Agent<Buffer>, graph::Error>;

pub struct Buffer {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Node,
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
    pub fn link(gl: &WGLRC, target: u32, array: &Node) -> Result {
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::make(|back| Self {
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
        self.array.read_or_error(|array| 
            unsafe {
                match array {
                    Load::Vf32(array) => 
                    self.gl.buffer_data_with_array_buffer_view(
                        self.target,
                        &Float32Array::view(array.as_slice()),
                        WGLRC::STATIC_DRAW,
                    ),
                    Load::Vu16(array) => 
                    self.gl.buffer_data_with_array_buffer_view(
                        self.target,
                        &Uint16Array::view(array.as_slice()),
                        WGLRC::STATIC_DRAW,
                    ),
                    _ => ()
                }
            }
        )?;
        self.unbind();
        Ok(Tray::None)
    }
}