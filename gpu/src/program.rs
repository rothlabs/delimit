use super::*;

pub type Result = std::result::Result<Agent<Program>, String>;

pub struct Program {
    target: WebGlProgram,
    vertex: Agent<Shader>,
    fragment: Agent<Shader>,
    gl: WGLRC,
}

impl Program {
    pub fn link(gl: &WGLRC, vertex: &Agent<Shader>, fragment: &Agent<Shader>) -> Result {
        let target = gl.create_program().ok_or("failed to create program")?;
        vertex.read(|unit| gl.attach_shader(&target, &unit.target));
        fragment.read(|unit| gl.attach_shader(&target, &unit.target));
        let link = Agent::make(|back| Self {
            gl: gl.clone(),
            vertex: vertex.backed(back),
            fragment: fragment.backed(back),
            target,
        });
        link.act()?;
        Ok(link)
    }
    pub fn use_target(&self) {
        self.gl.use_program(Some(&self.target));
    }
}

impl Act for Program {
    type Load = ReactResult;
    fn act(&self) -> Self::Load {
        self.vertex.act()?;
        self.fragment.act()?;
        self.gl.link_program(&self.target);
        if self
            .gl
            .get_program_parameter(&self.target, WGLRC::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(())
        } else {
            Err(self
                .gl
                .get_program_info_log(&self.target)
                .ok_or("failed to get program info log")?)
        }
    }
}

impl React for Program {
    fn react(&self, _: &Meta) -> ReactResult {
        self.act()
    }
}

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
