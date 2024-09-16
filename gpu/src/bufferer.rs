use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct Bufferer {
    buffer: Buffer,
    array: Apex,
}

impl BuffererBuilder {
    pub fn make(self) -> Result<Node<Bufferer>> {
        let mut buffer = self.build()?;
        Node::make(|back| {
            buffer.array = buffer.array.backed(back)?;
            Ok(buffer)
        })
    }
}

impl Bufferer {
    pub fn array(&mut self, array: impl Into<Apex>) {
        self.array = array.into();
    }
    fn size(&self, size: i32) {
        self.buffer
            .gl
            .buffer_data_with_i32(self.buffer.target, size, WGLRC::DYNAMIC_READ);
    }
    fn vec_u16(&self, array: &Vec<u16>) {
        let view = unsafe { Uint16Array::view(array.as_slice()) };
        self.buffer.gl.buffer_data_with_array_buffer_view(
            self.buffer.target,
            &view,
            WGLRC::DYNAMIC_DRAW,
        );
    }
    fn vec_f32(&self, array: &Vec<f32>) {
        let view = unsafe { Float32Array::view(array.as_slice()) };
        self.buffer.gl.buffer_data_with_array_buffer_view(
            self.buffer.target,
            &view,
            WGLRC::DYNAMIC_DRAW,
        );
    }
}

impl Act for Bufferer {
    async fn act(&self) -> Result<()> {
        self.buffer.bind();
        match &self.array {
            Apex::I32(size) => self.size(size.base().await?),
            Apex::Vu16(array) => array.read(|array| self.vec_u16(array)).await?,
            Apex::Vf32(array) => array.read(|array| self.vec_f32(array)).await?,
            _ => Err(anyhow!("wrong apex"))?,
        };
        self.buffer.unbind();
        Ok(())
    }
}

impl Reckon for Bufferer {}
