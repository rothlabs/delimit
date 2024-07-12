use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext as GL, WebGlProgram, WebGlShader};

pub struct GPU {
    gl: GL,
}

impl GPU {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn shader(&self, r#type: u32, source: &str) -> Result<WebGlShader, String> {
        let shader = self.gl.create_shader(r#type).ok_or("cannot create shader")?;
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);
        if self
            .gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(self
                .gl
                .get_shader_info_log(&shader)
                .ok_or("cannot get shader info log")?)
        }
    }
    pub fn program(&self, vertex: &WebGlShader, fragment: &WebGlShader) -> Result<WebGlProgram, String> {
        let program = self.gl.create_program().ok_or("cannot create program")?;
        self.gl.attach_shader(&program, vertex);
        self.gl.attach_shader(&program, fragment);
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
        let canvas = document.get_element_by_id("canvas").unwrap();
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
