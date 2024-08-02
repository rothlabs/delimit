use super::*;
use web_sys::WebGlProgram;

pub type Result = std::result::Result<Agent<Program>, graph::Error>;

/// GPU program based on vertex and fragment shaders.
pub struct Program {
    program: WebGlProgram,
    vertex: Agent<Shader>,
    fragment: Agent<Shader>,
    gl: WGLRC,
}

impl Program {
    pub fn link(gl: &WGLRC, vertex: &Agent<Shader>, fragment: &Agent<Shader>) -> Result {
        let program = gl.create_program().ok_or("failed to create program")?;
        vertex.read(|unit| gl.attach_shader(&program, &unit.shader));
        fragment.read(|unit| gl.attach_shader(&program, &unit.shader));
        let link = Agent::maker(|back| Self {
            gl: gl.clone(),
            vertex: vertex.backed(back),
            fragment: fragment.backed(back),
            program,
        });
        link.solve(Task::None)?;
        Ok(link)
    }
    pub fn use_(&self) {
        self.gl.use_program(Some(&self.program));
    }
}

impl Solve for Program {
    fn solve(&self, _: Task) -> solve::Result {
        self.vertex.solve(Task::None)?;
        self.fragment.solve(Task::None)?;
        self.gl.link_program(&self.program);
        if self
            .gl
            .get_program_parameter(&self.program, WGLRC::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Tray::None)
        } else {
            Err(self
                .gl
                .get_program_info_log(&self.program)
                .ok_or("failed to get program info log")?.into())
        }
    }
}

// impl React for Program {
//     fn react(&self, _: &Meta) -> react::Result {
//         self.act()
//     }
// }

// self.wglrc.attach_shader(&self.target, &self.vertex.act()?);
//         self.wglrc.attach_shader(&self.target, &self.fragment.act()?);

// impl Program {
//     // pub fn new(wglrc: &WGLRC) -> Self {
//     //     Self(wglrc.clone())
//     // }
//     // pub fn shader(&self) -> Shader {
//     //     Shader::new(&self.0)
//     // }
//     pub fn program(
//         &self,
//         vertex_shader: &WebGlShader,
//         fragment_shader: &WebGlShader,
//     ) -> Result<WebGlProgram, String> {
//         let program = self.0.create_program().ok_or("cannot create program")?;
//         self.0.attach_shader(&program, vertex_shader);
//         self.0.attach_shader(&program, fragment_shader);
//         self.0.link_program(&program);
//         if self
//             .0
//             .get_program_parameter(&program, WGLRC::LINK_STATUS)
//             .as_bool()
//             .unwrap_or(false)
//         {
//             Ok(program)
//         } else {
//             Err(self
//                 .0
//                 .get_program_info_log(&program)
//                 .ok_or("cannot get program info log")?)
//         }
//     }
// }

// impl Default for GPU {
//     fn default() -> Self {
//         let document = web_sys::window().unwrap().document().unwrap();
//         // let canvas = document.get_element_by_id("canvas").unwrap();
//         let canvas = document.create_element("canvas").unwrap();
//         // let canvas =
//         let canvas: web_sys::HtmlCanvasElement =
//             canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
//         let gl = canvas
//             .get_context("webgl2")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<GL>()
//             .unwrap();
//         Self { gl }
//     }
// }
