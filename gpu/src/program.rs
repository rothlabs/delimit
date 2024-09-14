use super::*;
use web_sys::WebGlProgram;

/// GPU program based on vertex and fragment shaders.
#[derive(Builder, Debug)]
#[builder(setter(into), build_fn(error = "graph::Error"))]
pub struct Program {
    gl: WGLRC,
    object: WebGlProgram,
    vertex: Node<Shader>,
    fragment: Node<Shader>,
    #[builder(default)]
    outs: Vec<Hub<String>>,
}

impl ProgramBuilder {
    pub fn make(&mut self) -> Result<Node<Program>> {
        let mut program = self.build()?;
        program
            .vertex
            .read(|unit| program.gl.attach_shader(&program.object, &unit.object))?;
        program
            .fragment
            .read(|unit| program.gl.attach_shader(&program.object, &unit.object))?;
        Node::make(|back| {
            program.vertex = program.vertex.backed(back)?;
            program.fragment = program.fragment.backed(back)?;
            Ok(program)
        })
    }
}

impl Program {
    pub fn use_(&self) {
        self.gl.use_program(Some(&self.object));
    }
}

impl Act for Program {
    fn act(&self) -> Result<()> {
        self.vertex.act()?;
        self.fragment.act()?;
        if !self.outs.is_empty() {
            let outs = Array::new();
            for out in &self.outs {
                outs.push(&out.base()?.into());
            }
            self.gl.transform_feedback_varyings(
                &self.object,
                &outs.into(),
                WGLRC::INTERLEAVED_ATTRIBS,
            );
        }
        self.gl.link_program(&self.object);
        if self
            .gl
            .get_program_parameter(&self.object, WGLRC::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            let memo = self
                .gl
                .get_program_info_log(&self.object)
                .unwrap_or("failed to get program info log".into());
            Err(anyhow!(memo))?
        }
    }
}

// let outs: Vec<String> = self.outputs.iter().map(Hub::base).collect()?;
// let outputs = Array::from_iter(outs.into_iter());
// let outputs = Array::from_iter(self.outputs.iter().map(|x| Hub::base));
