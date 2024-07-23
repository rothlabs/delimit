use super::*;

//pub type Array = array::Stem<f32>; 
pub type Array = Ploy<Ace<Array1<f32>>>;

pub struct Buffer {
    target: WebGlBuffer,
    array: Array,
    wglrc: WGLRC,
}

// impl Buffer {
//     pub fn link(wglrc: &WGLRC, array: &Array) -> BufferResult {
//         let target = wglrc.create_buffer().ok_or("failed to create buffer")?;
//         let link = Agent::make(|back| Self {
//             wglrc: wglrc.clone(),
//             array: array.backed(back),
//             target,
//         });
//         link.act()?;
//         Ok(link)
//     }
// }

// impl Act for Buffer {
//     type Load = Result<(), String>;
//     fn act(&self) -> Self::Load {
//         let array = self.array.grant();
//     }
// }