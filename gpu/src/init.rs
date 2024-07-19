use web_sys::{WebGl2RenderingContext as GL, WebGlShader};

pub struct Shader<'a> {
    gl: &'a GL,
}

impl<'a> Shader<'a> {
    pub fn new(gl: &'a GL) -> Self {
        Self { gl }
    }
    pub fn vertex(&self, source: &str) -> Result {
        self.create(GL::VERTEX_SHADER, source)
    }
    pub fn fragment(&self, source: &str) -> Result {
        self.create(GL::FRAGMENT_SHADER, source)
    }
    fn create(&self, r#type: u32, source: &str) -> Result {
        let shader = self
            .gl
            .create_shader(r#type)
            .ok_or("cannot create shader")?;
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);
        if self
            .gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .gl
                .get_shader_info_log(&shader)
                .ok_or("cannot get shader info log")?)
        }
    }
}

type Result = std::result::Result<WebGlShader, String>;