use super::*;
use web_sys::WebGlShader;

pub mod basic;

#[cfg(test)]
mod tests;

//pub type Source = Node;
pub type Result = std::result::Result<Agent<Shader>, graph::Error>;

pub struct Shader {
    pub gl: WGLRC,
    pub source: Node,
    pub shader: WebGlShader,
}

impl Shader {
    pub fn link(gl: &WGLRC, type_: u32, source: &Node) -> Result {
        let shader = gl.create_shader(type_).ok_or("failed to create shader")?;
        let link = Agent::make(|back| Self {
            gl: gl.clone(),
            source: source.backed(back),
            shader,
        });
        link.solve(Task::Main)?;
        Ok(link)
    }
}

impl Solve for Shader {
    fn solve(&self, _: Task) -> solve::Result {
        self.source
            .read_string(|src| self.gl.shader_source(&self.shader, src));
        self.gl.compile_shader(&self.shader);
        if self
            .gl
            .get_shader_parameter(&self.shader, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Gain::None)
        } else {
            Err(self
                .gl
                .get_shader_info_log(&self.shader)
                .ok_or("failed to get shader info log")?.into())
        }
    }
}