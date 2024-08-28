use super::*;
use web_sys::WebGlShader;

pub mod basic;

#[cfg(test)]
mod tests;

//pub type Source = Apex;
pub type Result = std::result::Result<Node<Shader>, graph::Error>;

#[derive(Debug)]
pub struct Shader {
    pub gl: WGLRC,
    pub source: Apex,
    pub shader: WebGlShader,
}

impl Shader {
    pub fn link(gl: &WGLRC, type_: u32, source: &Apex) -> Result {
        let shader = gl.create_shader(type_).ok_or(anyhow!("failed to create shader"))?;
        let link = Node::make(|back| Self {
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
            solve_ok()
        } else {
            let memo = self
                .gl
                .get_shader_info_log(&self.shader)
                .unwrap_or("failed to get shader info log".into());
            Err(anyhow!(memo))?
        }
    }
}