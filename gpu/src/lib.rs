pub use program::Program;
pub use canvas::Canvas;
pub use shader::Shader;
pub use buffer::Buffer;

use graph::*;
use text::*;
use mecha::*;
use web_sys::*;
use wasm_bindgen::prelude::*;

pub mod shader;

mod buffer;
mod program;
mod canvas;

pub type WGLRC = WebGl2RenderingContext;
pub type ShaderResult = Result<Agent<Shader>, String>;
pub type ProgramResult = Result<Agent<Program>, String>;
pub type BufferResult = Result<Agent<Buffer>, String>;

/// GPU graph maker
pub struct Gpu {
    pub wglrc: WGLRC
}

impl From<WGLRC> for Gpu {
    fn from(wglrc: WGLRC) -> Self {
        Self { wglrc }
    }
}

impl Gpu {
    pub fn vertex_shader(&self, source: impl Into<plain::Stem>) -> ShaderResult {
        Shader::link(&self.wglrc, WGLRC::VERTEX_SHADER, source.into())
    }
    pub fn fragment_shader(&self, source: impl Into<plain::Stem>) -> ShaderResult {
        Shader::link(&self.wglrc, WGLRC::FRAGMENT_SHADER, source.into())
    }
    pub fn program(&self, vertex: &Agent<Shader>, fragment: &Agent<Shader>) -> ProgramResult {
        Program::link(&self.wglrc, vertex, fragment)
    }
}

// let wglrc = canvas
// .get_context("webgl2")
// .unwrap()
// .unwrap()
// .dyn_into::<WGLRC>()
// .unwrap();
// Self { wglrc }

// impl Default for Gpu {
//     fn default() -> Self {
//         let document = window().unwrap().document().unwrap();
//         let canvas = document
//             .create_element("canvas")
//             .unwrap()
//             .dyn_into::<HtmlCanvasElement>()
//             .unwrap();
//         let wglrc = canvas
//             .get_context("webgl2")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<WGLRC>()
//             .unwrap();
//         Self { canvas, wglrc }
//     }
// }
