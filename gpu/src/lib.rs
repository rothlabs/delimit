pub use buffer::Buffer;
pub use canvas::Canvas;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use buffer::Array;
use elements::*;
use graph::*;
use js_sys::*;
use mecha::*;
use shader::*;
use vao::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod shader;

mod buffer;
mod canvas;
mod elements;
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
    pub fn array_buffer(&self, array: impl Into<Array<f32>>) -> buffer::Result<f32> {
        Buffer::link_f32(&self.wglrc, WGLRC::ARRAY_BUFFER, &array.into())
    }
    pub fn element_buffer(&self, array: impl Into<Array<u16>>) -> buffer::Result<u16> {
        Buffer::link_u16(&self.wglrc, WGLRC::ELEMENT_ARRAY_BUFFER, &array.into())
    }
    pub fn vertex_attribute(&self, buffer: &Agent<Buffer<f32>>) -> Agent<VertexAttribute> {
        VertexAttribute::link(&self.wglrc, buffer)
    }
    pub fn vao(&self, attributes: &Attributes) -> vao::Result {
        Vao::link(&self.wglrc, attributes)
    }
    pub fn elements(
        &self,
        program: &Agent<Program>,
        buffer: &Agent<Buffer<f32>>,
        vao: &Agent<Vao>,
    ) -> Agent<Elements> {
        Elements::link(&self.wglrc, program, buffer, vao)
    }
}
