use super::*;

pub struct Base<'a>(&'a WGLRC);

impl<'a> Base<'a> {
    pub fn new(wglrc: &'a WGLRC) -> Self {
        Self(wglrc)
    }
    pub fn shader(&self) -> Shader {
        Shader::new(self.0)
    }
    pub fn program(
        &self,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self.0.create_program().ok_or("cannot create program")?;
        self.0.attach_shader(&program, vertex_shader);
        self.0.attach_shader(&program, fragment_shader);
        self.0.link_program(&program);
        if self
            .0
            .get_program_parameter(&program, WGLRC::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self
                .0
                .get_program_info_log(&program)
                .ok_or("cannot get program info log")?)
        }
    }
}

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