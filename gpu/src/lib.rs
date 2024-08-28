pub use anyhow::anyhow;
pub use buffer::Buffer;
pub use canvas::Canvas;
pub use elements::Elements;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use derive_builder::Builder;
use elements::ElementsBuilder;
use graph::*;
use texture::*;
use vao::*;
use vertex_attribute::VertexAttributeBuilder;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::*, WebGl2RenderingContext};

pub mod buffer;
pub mod program;
pub mod shader;
pub mod texture;
pub mod vao;

mod canvas;
mod elements;
mod vertex_attribute;

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
    pub fn vertex_shader(&self, source: impl Into<Apex>) -> shader::Result {
        Shader::link(&self.gl, WGLRC::VERTEX_SHADER, &source.into())
    }
    pub fn fragment_shader(&self, source: impl Into<Apex>) -> shader::Result {
        Shader::link(&self.gl, WGLRC::FRAGMENT_SHADER, &source.into())
    }
    pub fn program(&self, vertex: &Node<Shader>, fragment: &Node<Shader>) -> program::Result {
        Program::link(&self.gl, vertex, fragment)
    }
    pub fn buffer(&self, array: impl Into<Apex>) -> buffer::Result {
        // f32
        Buffer::link(&self.gl, WGLRC::ARRAY_BUFFER, &array.into())
    }
    pub fn index_buffer(&self, array: impl Into<Apex>) -> buffer::Result {
        // u16
        Buffer::link(&self.gl, WGLRC::ELEMENT_ARRAY_BUFFER, &array.into())
    }
    pub fn vertex_attribute(&self, buffer: &Node<Buffer>) -> VertexAttributeBuilder {
        // f32
        VertexAttributeBuilder::default()
            .gl(self.gl.clone())
            .buffer(buffer.clone())
            .clone()
    }
    pub fn vao(&self, attributes: &Attributes) -> GraphResult<VaoBuilder> {
        let object = self
            .gl
            .create_vertex_array()
            .ok_or(anyhow!("failed to create vertex array object"))?;
        Ok(VaoBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .attributes(attributes.clone())
            .clone())
    }
    pub fn texture(
        // <T: Copy>
        &self,
        array: impl Into<Apex>,
    ) -> GraphResult<TextureBuilder> {
        let texture = self
            .gl
            .create_texture()
            .ok_or(anyhow!("failed to create texture"))?;
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&texture));
        self.default_texture_filters();
        Ok(TextureBuilder::default()
            .gl(self.gl.clone())
            .texture(texture)
            .array(array)
            .clone())
    }
    pub fn elements(&self, program: &Node<Program>) -> ElementsBuilder {
        ElementsBuilder::default()
            .gl(self.gl.clone())
            .program(program.clone())
            .clone()
        // Elements::link(&self.gl, program, buffer, vao)
    }
    fn default_texture_filters(&self) {
        self.default_texture_min_filter();
        self.default_texture_mag_filter();
    }
    fn default_texture_min_filter(&self) {
        self.gl.tex_parameteri(
            WGLRC::TEXTURE_2D,
            WGLRC::TEXTURE_MIN_FILTER,
            WGLRC::NEAREST as i32,
        );
    }
    fn default_texture_mag_filter(&self) {
        self.gl.tex_parameteri(
            WGLRC::TEXTURE_2D,
            WGLRC::TEXTURE_MAG_FILTER,
            WGLRC::NEAREST as i32,
        );
    }
}

// <F: FnOnce(&mut VertexAttribute)>
// pub fn vertex_attribute(&self, buffer: &Node<Buffer<f32>>) -> Node<VertexAttribute> {
//     VertexAttribute::link(&self.gl, buffer)
// }
