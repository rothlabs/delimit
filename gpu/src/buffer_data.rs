use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"), setter(into))]
pub struct BufferData {
    gl: WGLRC,
    buffer: Buffer,
    array: Apex,
}

impl BufferDataBuilder {
    pub fn make(self) -> Result<Node<BufferData>> {
        let mut buffer = self.build()?;
        Node::make(|back| {
            buffer.array = buffer.array.backed(back)?;
            Ok(buffer)
        })
    }
}

impl BufferData {
    pub fn array(&mut self, array: impl Into<Apex>) {
        self.array = array.into();
    }
    fn vec_u16(&self, array: &Vec<u16>) -> Result<()> {
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.buffer.target,
                &Uint16Array::view(array.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        }
        Ok(())
    }
    fn vec_f32(&self, array: &Vec<f32>) -> Result<()> {
        unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.buffer.target,
                &Float32Array::view(array.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        }
        Ok(())
    }
}

impl Act for BufferData {
    fn act(&self) -> Result<()> {
        self.buffer.bind();
        match &self.array {
            Apex::Vu16(array) => array.read(|array| self.vec_u16(array))?,
            Apex::Vf32(array) => array.read(|array| self.vec_f32(array))?,
            _ => Err(anyhow!("wrong apex"))?,
        }?;
        self.buffer.unbind();
        Ok(())
    }
}