use super::*;
use web_sys::WebGlShader;

pub mod basic;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Shader {
    pub gl: WGLRC,
    pub object: WebGlShader,
    pub source: Hub<String>,
}

impl Shader {
    pub fn make(gl: &WGLRC, type_: u32, source: &Hub<String>) -> Result<Node<Shader>> {
        let shader = gl
            .create_shader(type_)
            .ok_or(anyhow!("failed to create shader"))?;
        Self {
            gl: gl.clone(),
            source: source.clone(),
            object: shader,
        }
        .node()
    }
}

impl Act for Shader {
    async fn act(&self) -> Result<()> {
        self.source
            .read(|src| self.gl.shader_source(&self.object, src))
            .await?;
        self.gl.compile_shader(&self.object);
        if self
            .gl
            .get_shader_parameter(&self.object, WGLRC::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            let memo = self
                .gl
                .get_shader_info_log(&self.object)
                .unwrap_or("failed to get shader info log".into());
            Err(anyhow!(memo))?
        }
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.source.back(back)
    }
}
