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

// impl Make for Buffer {
//     fn make(&self, back: &Back) -> Self {
//         Self {
//             gl: gl.clone(),
//             buffer,
//             target,
//             array: array.backed(back),
//         }
//     }
// }

impl Buffer {
    pub fn link(gl: &WGLRC, target: u32, array: &Node) -> Result {
        let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
        let link = Agent::maker(|back| Self {
            gl: gl.clone(),
            buffer,
            target,
            array: array.backed(back),
        });
        link.solve(Task::None)?;
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

// impl React for Buffer {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.solve(Task::Main);
//         Ok(())
//     }
// }









// impl Buffer<f32> {
//     pub fn link_f32(gl: &WGLRC, target: u32, array: &Node) -> Result<f32> {
//         let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
//         let link = Agent::make(|back| Self {
//             gl: gl.clone(),
//             buffer,
//             target,
//             array: array.backed(back),
//         });
//         link.act();
//         Ok(link)
//     }
// }

// impl Buffer<u16> {
//     pub fn link_u16(wglrc: &WGLRC, target: u32, array: &Node) -> Result<u16> {
//         let buffer = wglrc.create_buffer().ok_or("failed to create buffer")?;
//         let link = Agent::make(|back| Self {
//             gl: wglrc.clone(),
//             target,
//             buffer,
//             array: array.backed(back),
//         });
//         link.act();
//         Ok(link)
//     }
// }

// impl Act for Buffer<f32> {
//     type Load = ();
//     fn act(&self) -> Self::Load {
//         self.bind();
//         self.array.read_vf32(|unit| unsafe {
//             self.gl.buffer_data_with_array_buffer_view(
//                 self.target,
//                 &Float32Array::view(unit.as_slice()),
//                 WGLRC::STATIC_DRAW,
//             )
//         });
//         self.unbind();
//     }
// }

// impl Act for Buffer<u16> {
//     type Load = ();
//     fn act(&self) -> Self::Load {
//         self.bind();
//         self.array.read_vu16(|unit| unsafe {
//             self.gl.buffer_data_with_array_buffer_view(
//                 self.target,
//                 &Uint16Array::view(unit.as_slice()),
//                 WGLRC::STATIC_DRAW,
//             )
//         });
//         self.unbind();
//     }
// }

// impl React for Buffer<f32> {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act();
//         Ok(())
//     }
// }

// impl React for Buffer<u16> {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act();
//         Ok(())
//     }
// }











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
