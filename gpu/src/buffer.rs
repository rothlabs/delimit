use super::*;

pub type Array = AcePloy<Array1D<f32>>;
pub type Result = std::result::Result<Agent<Buffer>, String>;

pub struct Buffer {
    gl: WGLRC,
    target: WebGlBuffer,
    kind: u32,
    array: Array,
}

impl Buffer {
    pub fn link(wglrc: &WGLRC, kind: u32, array: &Array) -> Result {
        let target = wglrc.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::make(|back| Self {
            gl: wglrc.clone(),
            target,
            kind,
            array: array.backed(back),
        });
        link.act();
        Ok(link)
    }
}

impl Act for Buffer {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.gl.bind_buffer(self.kind, Some(&self.target));
        self.array.grant().read(|unit| unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.kind,
                &Float32Array::view(unit.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        })
    }
}
