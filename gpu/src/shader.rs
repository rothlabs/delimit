use super::*;

pub mod basic;

#[cfg(test)]
mod tests;

pub type Source = Value<String>;
pub type Result = std::result::Result<Agent<Shader>, String>;

pub struct Shader {
    pub gl: WGLRC,
    pub source: Source,
    pub shader: WebGlShader,
}

impl Shader {
    pub fn link(gl: &WGLRC, type_: u32, source: &Source) -> Result {
        let shader = gl.create_shader(type_).ok_or("failed to create shader")?;
        let link = Agent::make(|back| Self {
            gl: gl.clone(),
            source: source.backed(back),
            shader,
        });
        link.act()?;
        Ok(link)
    }
}

impl Act for Shader {
    type Load = react::Result;
    fn act(&self) -> Self::Load {
        self.source
            .read(|src| self.gl.shader_source(&self.shader, src));
        self.gl.compile_shader(&self.shader);
        if self
            .gl
            .get_shader_parameter(&self.shader, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(self
                .gl
                .get_shader_info_log(&self.shader)
                .ok_or("failed to get shader info log")?)
        }
    }
}

impl React for Shader {
    fn react(&self, _: &Meta) -> react::Result {
        self.act()
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
