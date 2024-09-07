use super::*;
use web_sys::WebGlShader;

pub mod basic;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Shader {
    pub gl: WGLRC,
    pub source: Hub,
    pub shader: WebGlShader,
}

impl Shader {
    pub fn make(gl: &WGLRC, type_: u32, source: &Hub) -> Result<Node<Shader>> {
        let shader = gl
            .create_shader(type_)
            .ok_or(anyhow!("failed to create shader"))?;
        let node = Node::make(|back| {
            let unit = Self {
                gl: gl.clone(),
                source: source.backed(back)?,
                shader,
            };
            Ok(unit)
        })?;
        node.act()?;
        Ok(node)
    }
}

impl Act for Shader {
    fn act(&self) -> graph::Result<()> {
        self.source
            .view()
            .string(|src| self.gl.shader_source(&self.shader, src))?;
        self.gl.compile_shader(&self.shader);
        if self
            .gl
            .get_shader_parameter(&self.shader, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            let memo = self
                .gl
                .get_shader_info_log(&self.shader)
                .unwrap_or("failed to get shader info log".into());
            Err(anyhow!(memo))?
        }
    }
}

impl Adapt for Shader {
    fn adapt(&mut self, _: &mut dyn Deal) -> Result<()> {
        Ok(())
    }
}
