pub use buffer::Buffer;
pub use canvas::Canvas;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use vao::*;
use graph::*;
use js_sys::*;
use mecha::*;
use shader::*;
use buffer::Array;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod shader;

mod buffer;
mod canvas;
mod program;
mod vao;
mod vertex_attribute;

pub type WGLRC = WebGl2RenderingContext;

/// GPU graph maker
pub struct Gpu {
    pub wglrc: WGLRC,
}

impl From<WGLRC> for Gpu {
    fn from(wglrc: WGLRC) -> Self {
        Self { wglrc }
    }
}

impl Gpu {
    pub fn vertex_shader(&self, source: impl Into<Source>) -> shader::Result {
        Shader::link(&self.wglrc, WGLRC::VERTEX_SHADER, &source.into())
    }
    pub fn fragment_shader(&self, source: impl Into<Source>) -> shader::Result {
        Shader::link(&self.wglrc, WGLRC::FRAGMENT_SHADER, &source.into())
    }
    pub fn program(&self, vertex: &Agent<Shader>, fragment: &Agent<Shader>) -> program::Result {
        Program::link(&self.wglrc, vertex, fragment)
    }
    pub fn array_buffer(&self, array: impl Into<Array>) -> buffer::Result {
        Buffer::link(&self.wglrc, WGLRC::ARRAY_BUFFER, &array.into())
    }
    pub fn vertex_attribute(&self) -> Agent<VertexAttribute> {
        VertexAttribute::link(&self.wglrc)
    }
    pub fn vao(&self, attributes:  &Attributes) -> vao::Result {
        Vao::link(&self.wglrc, attributes)
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
