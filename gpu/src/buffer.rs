use super::*;

pub type Array<T> = Asset<Vec<T>>;
pub type Result<T> = std::result::Result<Agent<Buffer<T>>, String>;

pub struct Buffer<T> {
    gl: WGLRC,
    target: WebGlBuffer,
    kind: u32,
    array: Array<T>,
}

impl<T> Buffer<T> {
    pub fn bind(&self) {
        self.gl.bind_buffer(self.kind, Some(&self.target));
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.kind, None);
    }
}

impl Buffer<f32> {
    pub fn link_f32(gl: &WGLRC, kind: u32, array: &Array<f32>) -> Result<f32> {
        let target = gl.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::make(|back| Self {
            gl: gl.clone(),
            target,
            kind,
            array: array.backed(back),
        });
        link.act();
        Ok(link)
    }
}

impl Buffer<u16> {
    pub fn link_u16(wglrc: &WGLRC, kind: u32, array: &Array<u16>) -> Result<u16> {
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

impl Act for Buffer<f32> {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.bind();
        self.array.grant().read(|unit| unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.kind,
                &Float32Array::view(unit.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        });
        self.unbind();
    }
}

impl Act for Buffer<u16> {
    type Load = ();
    fn act(&self) -> Self::Load {
        self.bind();
        self.array.grant().read(|unit| unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.kind,
                &Uint16Array::view(unit.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        });
        self.unbind();
    }
}

impl React for Buffer<f32> {
    fn react(&self, _: &Meta) -> ReactResult {
        self.act();
        Ok(())
    }
}

impl React for Buffer<u16> {
    fn react(&self, _: &Meta) -> ReactResult {
        self.act();
        Ok(())
    }
}
