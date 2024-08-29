use super::*;
use web_sys::WebGlProgram;

pub type Result = std::result::Result<Node<Program>, graph::Error>;

/// GPU program based on vertex and fragment shaders.
#[derive(Debug)]
pub struct Program {
    program: WebGlProgram,
    vertex: Node<Shader>,
    fragment: Node<Shader>,
    gl: WGLRC,
}

impl Program {
    pub fn link(gl: &WGLRC, vertex: &Node<Shader>, fragment: &Node<Shader>) -> Result {
        let program = gl
            .create_program()
            .ok_or(anyhow!("failed to create program"))?;
        vertex.read(|unit| gl.attach_shader(&program, &unit.shader))?;
        fragment.read(|unit| gl.attach_shader(&program, &unit.shader))?;
        let link = Node::make(|back| Self {
            gl: gl.clone(),
            vertex: vertex.backed(back),
            fragment: fragment.backed(back),
            program,
        });
        link.solve(Task::Main)?;
        Ok(link)
    }
    pub fn use_(&self) {
        self.gl.use_program(Some(&self.program));
    }
}

impl Solve for Program {
    fn solve(&self, _: Task) -> solve::Result {
        self.vertex.solve(Task::Main)?;
        self.fragment.solve(Task::Main)?;
        self.gl.link_program(&self.program);
        if self
            .gl
            .get_program_parameter(&self.program, WGLRC::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            solve_ok()
        } else {
            let memo = self
                .gl
                .get_program_info_log(&self.program)
                .unwrap_or("failed to get program info log".into());
            Err(anyhow!(memo))?
        }
    }
}
