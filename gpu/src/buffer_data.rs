use super::*;

#[derive(Debug)]
pub struct BufferData {
    pub gl: WGLRC,
    pub buffer: Node<Buffer>,
    pub array: Apex,
}

// #[derive(Builder, Debug)]
// #[builder(build_fn(error = "graph::Error"))]

// impl BufferDataBuilder {
//     pub fn make(&self) -> Result<Node<BufferData>> {
//         let mut buffer = self.build()?;
//         Node::make(|back| {
//             buffer.array = buffer.array.backed(back)?;
//             Ok(buffer)
//         })
//     }
// }

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


// pub fn bind(&self) {
//     self.gl.bind_buffer(self.target, Some(&self.object));
// }
// pub fn unbind(&self) {
//     self.gl.bind_buffer(self.target, None);
// }
// pub fn bind_base(&self) {
//     self.gl.bind_buffer_base(self.target, 0, Some(&self.object));
// }
// // pub fn unbind_base(&self) {
// //     self.gl.bind_buffer_base(self.target, 0, None);
// // }


// pub fn make(gl: &WGLRC, target: u32, array: &Apex) -> Result<Node<Buffer>> {
//     let object = gl
//         .create_buffer()
//         .ok_or(anyhow!("failed to create buffer"))?;
//     Node::make(|back| {
//         let buffer = Self {
//             gl: gl.clone(),
//             object,
//             target,
//             array: array.backed(back)?,
//         };
//         Ok(buffer)
//     })
// }