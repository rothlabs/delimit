use super::*;
use web_sys::WebGlBuffer;

pub type Result<T> = std::result::Result<Agent<Buffer<T>>, String>;

pub struct Buffer<T> {
    gl: WGLRC,
    buffer: WebGlBuffer,
    target: u32,
    array: Array<T>,
}

impl<T> Buffer<T> {
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer));
    }
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }
}

impl Buffer<f32> {
    pub fn link_f32(gl: &WGLRC, target: u32, array: &Array<f32>) -> Result<f32> {
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::make(|back| Self {
            gl: gl.clone(),
            buffer,
            target,
            array: array.backed(back),
        });
        link.act();
        Ok(link)
    }
}

impl Buffer<u16> {
    pub fn link_u16(wglrc: &WGLRC, target: u32, array: &Array<u16>) -> Result<u16> {
        let buffer = wglrc.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::make(|back| Self {
            gl: wglrc.clone(),
            target,
            buffer,
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
        self.array.read(|unit| unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.target,
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
        self.array.read(|unit| unsafe {
            self.gl.buffer_data_with_array_buffer_view(
                self.target,
                &Uint16Array::view(unit.as_slice()),
                WGLRC::STATIC_DRAW,
            )
        });
        self.unbind();
    }
}

impl React for Buffer<f32> {
    fn react(&self, _: &Meta) -> react::Result {
        self.act();
        Ok(())
    }
}

impl React for Buffer<u16> {
    fn react(&self, _: &Meta) -> react::Result {
        self.act();
        Ok(())
    }
}

// // impl<T> Act for Buffer<T>
// // where
// //     T: ToArrayF32,
// // {
// //     type Load = ();
// //     fn act(&self) -> Self::Load {
// //         self.bind();
// //         self.array.grant().read(|unit| unsafe {
// //             self.gl.buffer_data_with_array_buffer_view(
// //                 self.target,
// //                 &Float32Array::view(unit.as_slice()),
// //                 WGLRC::STATIC_DRAW,
// //             )
// //         });
// //         self.unbind();
// //     }
// // }

// pub trait ToJsArray<T> {
//     fn array_f32(&self) -> Float32Array;
// }

// impl ToJsArray for &Vec<f32> {
//     fn array_f32(&self) -> Float32Array {
//         unsafe {
//             Float32Array::view(self.as_slice())
//         }
//     }
// }
