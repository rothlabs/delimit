use super::*;

pub mod basic;

pub struct Shader<'a>(&'a WGLRC);

impl<'a> Shader<'a> {
    pub fn new(wglrc: &'a WGLRC) -> Self {
        Self(wglrc)
    }
    pub fn vertex(&self, source: &str) -> Result {
        self.create(WGLRC::VERTEX_SHADER, source)
    }
    pub fn fragment(&self, source: &str) -> Result {
        self.create(WGLRC::FRAGMENT_SHADER, source)
    }
    fn create(&self, r#type: u32, source: &str) -> Result {
        let shader = self.0.create_shader(r#type).ok_or("cannot create shader")?;
        self.0.shader_source(&shader, source);
        self.0.compile_shader(&shader);
        if self
            .0
            .get_shader_parameter(&shader, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .0
                .get_shader_info_log(&shader)
                .ok_or("cannot get shader info log")?)
        }
    }
}

type Result = std::result::Result<WebGlShader, String>;
