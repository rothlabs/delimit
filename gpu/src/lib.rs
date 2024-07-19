use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

pub mod shader;

mod init;

pub struct GPU {
    gl: GL,
}

impl GPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shader(&self) -> init::Shader {
        init::Shader::new(&self.gl)
    }
    pub fn program(
        &self,
        vertex_shader: &WebGlShader,
        fragment_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = self.gl.create_program().ok_or("cannot create program")?;
        self.gl.attach_shader(&program, vertex_shader);
        self.gl.attach_shader(&program, fragment_shader);
        self.gl.link_program(&program);
        if self
            .gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(self
                .gl
                .get_program_info_log(&program)
                .ok_or("cannot get program info log")?)
        }
    }
}

impl Default for GPU {
    fn default() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        // let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas = document.create_element("canvas").unwrap();
        // let canvas = 
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let gl = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<GL>()
            .unwrap();
        Self { gl }
    }
}
