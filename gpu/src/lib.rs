pub use buffer::Buffer;
pub use canvas::Canvas;
pub use elements::Elements;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use buffer::Array;
use texture::*;
use graph::*;
use js_sys::*;
use shader::*;
use vao::*;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod shader;
pub mod buffer;
pub mod vao;
pub mod program;

mod canvas;
mod elements;
mod vertex_attribute;
mod texture;

pub type WGLRC = WebGl2RenderingContext;

/// GPU graph maker
pub struct Gpu {
    pub gl: WGLRC,
}

impl From<WGLRC> for Gpu {
    fn from(wglrc: WGLRC) -> Self {
        Self { gl: wglrc }
    }
}

impl Gpu {
    pub fn vertex_shader(&self, source: impl Into<Source>) -> shader::Result {
        Shader::link(&self.gl, WGLRC::VERTEX_SHADER, &source.into())
    }
    pub fn fragment_shader(&self, source: impl Into<Source>) -> shader::Result {
        Shader::link(&self.gl, WGLRC::FRAGMENT_SHADER, &source.into())
    }
    pub fn program(&self, vertex: &Agent<Shader>, fragment: &Agent<Shader>) -> program::Result {
        Program::link(&self.gl, vertex, fragment)
    }
    pub fn array_buffer(&self, array: impl Into<Array<f32>>) -> buffer::Result<f32> {
        Buffer::link_f32(&self.gl, WGLRC::ARRAY_BUFFER, &array.into())
    }
    pub fn element_buffer(&self, array: impl Into<Array<u16>>) -> buffer::Result<u16> {
        Buffer::link_u16(&self.gl, WGLRC::ELEMENT_ARRAY_BUFFER, &array.into())
    }
    // <F: FnOnce(&mut VertexAttribute)> 
    pub fn vertex_attribute(&self, buffer: &Agent<Buffer<f32>>) -> Agent<VertexAttribute> {
        VertexAttribute::link(&self.gl, buffer)
    }
    pub fn vao(&self, attributes: &Attributes) -> vao::Result {
        Vao::link(&self.gl, attributes)
    }
    pub fn texture(&self, array: impl Into<Array<u8>>) -> texture::Result<u8> {
        Texture::link_u8(&self.gl, &array.into())
    }
    pub fn elements(
        &self,
        program: &Agent<Program>,
        buffer: &Agent<Buffer<f32>>,
        vao: &Agent<Vao>,
    ) -> Agent<Elements> {
        Elements::link(&self.gl, program, buffer, vao)
    }
}
