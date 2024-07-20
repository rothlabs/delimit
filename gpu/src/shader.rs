use super::*;
use graph::*;
use text::*;

pub mod basic;

pub struct Shader {
    wglrc: WGLRC,
    shader: WebGlShader,
    source: plain::Role,
    // kind: u32,
}

// impl Shader {
//     pub fn new(wglrc: &WGLRC, kind: u32) -> Self {
//         Self { 
//             wglrc: wglrc.clone(),
//             kind,
//         }shader
// }

impl Grant for Shader {
    type Load = Result;
    fn grant(&self) -> Self::Load {
        // let shader = self.wglrc.create_shader(self.kind).ok_or("cannot create shader")?;
        let source = self.source.grant();
        source.read(|src| self.wglrc.shader_source(&self.shader, src));
        self.wglrc.compile_shader(&self.shader);
        if self
            .wglrc
            .get_shader_parameter(&self.shader, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(self
                .wglrc
                .get_shader_info_log(&self.shader)
                .ok_or("cannot get shader info log")?)
        }
    }
}

type Result = std::result::Result<(), String>;


// impl Shader {
//     pub fn new(wglrc: &WGLRC, kind: u32) -> Self {
//         Self { 
//             wglrc: wglrc.clone(),
//             kind,
//         }
//     }
//     pub fn vertex(&self, source: &str) -> Result {
//         self.create(WGLRC::VERTEX_SHADER, source)
//     }
//     pub fn fragment(&self, source: &str) -> Result {
//         self.create(WGLRC::FRAGMENT_SHADER, source)
//     }
//     fn create(&self, r#type: u32, source: &str) -> Result {
//         let shader = self.wglrc.create_shader(r#type).ok_or("cannot create shader")?;
//         self.wglrc.shader_source(&shader, source);
//         self.wglrc.compile_shader(&shader);
//         if self
//             .wglrc
//             .get_shader_parameter(&shader, WGLRC::COMPILE_STATUS)
//             .as_bool()
//             .unwrap_or(false)
//         {
//             Ok(shader)
//         } else {
//             Err(self
//                 .wglrc
//                 .get_shader_info_log(&shader)
//                 .ok_or("cannot get shader info log")?)
//         }
//     }
// }
