use super::*;
// use derive_builder::Builder;
use graph::*;
use text::*;

pub mod basic;

#[cfg(test)]
mod tests;

// #[derive(Builder)]
pub struct Shader {
    target: WebGlShader,
    source: plain::Role,
    wglrc: WGLRC,
}

impl Act for Shader {
    type Load = Result<(), String>;
    fn act(&self) -> Self::Load {
        // Ok(())
        // let shader = self.wglrc.create_shader(self.kind).ok_or("cannot create shader")?;
        let source = self.source.grant();
        source.read(|src| self.wglrc.shader_source(&self.target, src));
        self.wglrc.compile_shader(&self.target);
        if self
            .wglrc
            .get_shader_parameter(&self.target, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(()) // self.target.clone()
        } else {
            Err(self
                .wglrc
                .get_shader_info_log(&self.target)
                .ok_or("cannot get shader info log")?)
        }
    }
}

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

// impl Shader {
//     pub fn new(wglrc: &WGLRC, kind: u32) -> Self {
//         Self {
//             wglrc: wglrc.clone(),
//             kind,
//         }shader
// }
