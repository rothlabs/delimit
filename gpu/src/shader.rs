use super::*;

pub mod basic;

#[cfg(test)]
mod tests;

pub type Source = AceView<String>;
pub type Result = std::result::Result<Agent<Shader>, String>;

pub struct Shader {
    pub wglrc: WGLRC,
    pub source: Source,
    pub target: WebGlShader,
}

impl Shader {
    pub fn link(wglrc: &WGLRC, kind: u32, source: &Source) -> Result {
        let target = wglrc.create_shader(kind).ok_or("failed to create shader")?;
        let link = Agent::make(|back| Self {
            wglrc: wglrc.clone(),
            source: source.backed(back),
            target,
        });
        link.act()?;
        Ok(link)
    }
}

impl Act for Shader {
    type Load = std::result::Result<(), String>; // Ace<WebGlShader>
    fn act(&self) -> Self::Load {        

        self.source
            .read(|src| self.wglrc.shader_source(&self.target, src));
        self.wglrc.compile_shader(&self.target);
        if self
            .wglrc
            .get_shader_parameter(&self.target, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(()) // self.target.ace()
        } else {
            Err(self
                .wglrc
                .get_shader_info_log(&self.target)
                .ok_or("failed to get shader info log")?)
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
