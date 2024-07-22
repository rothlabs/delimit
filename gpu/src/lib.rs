pub use program::Program;
pub use canvas::Canvas;
pub use shader::Shader;

use web_sys::*;
use wasm_bindgen::prelude::*;

pub mod shader;

mod program;
mod canvas;

pub type WGLRC = WebGl2RenderingContext;

/// GPU graph maker
pub struct Gpu {
    pub canvas: HtmlCanvasElement,
    pub wglrc: WGLRC
}

impl Gpu {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Gpu {
    fn default() -> Self {
        let document = window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap();
        let wglrc = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WGLRC>()
            .unwrap();
        Self { canvas, wglrc }
    }
}
