pub use anyhow::anyhow;
pub use buffer::Buffer;
use buffer::BufferBuilder;
pub use buffer_in::BufferIn;
use buffer_in::BufferInBuilder;
pub use bufferer::Bufferer;
use bufferer::BuffererBuilder;
pub use canvas::Canvas;
pub use draw_arrays::DrawArrays;
pub use draw_elements::DrawElements;
pub use program::Program;
pub use shader::Shader;
pub use tfo::Tfo;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use derive_builder::Builder;
use draw_arrays::DrawArraysBuilder;
use draw_elements::DrawElementsBuilder;
use graph::*;
use program::ProgramBuilder;
use texture::*;
use tfo::TfoBuilder;
use vao::*;
use vertex_attribute::VertexAttributeBuilder;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::*, WebGl2RenderingContext, WebGlBuffer};

pub mod buffer;
pub mod buffer_in;
pub mod bufferer;
pub mod program;
pub mod shader;
pub mod texture;
pub mod tfo;
pub mod vao;

mod canvas;
mod draw_arrays;
mod draw_elements;
mod vertex_attribute;

pub type WGLRC = WebGl2RenderingContext;

#[derive(Debug, Clone)]
pub struct Gpu {
    pub gl: WGLRC,
}

impl From<WGLRC> for Gpu {
    fn from(wglrc: WGLRC) -> Self {
        Self { gl: wglrc }
    }
}

impl Gpu {
    pub fn vertex_shader(&self, source: impl Into<Hub<String>>) -> Result<Node<Shader>> {
        Shader::make(&self.gl, WGLRC::VERTEX_SHADER, &source.into())
    }
    pub fn fragment_shader(&self, source: impl Into<Hub<String>>) -> Result<Node<Shader>> {
        Shader::make(&self.gl, WGLRC::FRAGMENT_SHADER, &source.into())
    }
    pub fn program(&self, vertex: Node<Shader>, fragment: Node<Shader>) -> Result<ProgramBuilder> {
        let object = self
            .gl
            .create_program()
            .ok_or(anyhow!("failed to create program"))?;
        Ok(ProgramBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .vertex(vertex)
            .fragment(fragment)
            .clone())
    }
    pub fn buffer(&self) -> Result<Buffer> {
        let buffer = self
            .gl
            .create_buffer()
            .ok_or(anyhow!("create buffer failed"))?;
        let buffer = BufferBuilder::default()
            .gl(self.gl.clone())
            .object(buffer)
            .build()?;
        Ok(buffer)
    }
    pub fn vao(&self) -> Result<VaoBuilder> {
        let object = self
            .gl
            .create_vertex_array()
            .ok_or(anyhow!("failed to create vertex array object"))?;
        let builder = VaoBuilder::default()
            .gl(self.gl.clone())
            .object(object);
        Ok(builder)
    }
    pub fn tfo(&self) -> Result<TfoBuilder> {
        let object = self
            .gl
            .create_transform_feedback()
            .ok_or(anyhow!("failed to create transform feedback object"))?;
        let tfo = TfoBuilder::default()
            .gl(self.gl.clone())
            .object(object);
        Ok(tfo)
    }
    pub fn texture(&self, array: impl Into<Apex>) -> Result<TextureBuilder> {
        let object = self
            .gl
            .create_texture()
            .ok_or(anyhow!("failed to create texture"))?;
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&object));
        self.default_texture_filters();
        Ok(TextureBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .array(array)
            .clone())
    }
    pub fn draw_arrays(&self, program: Node<Program>) -> DrawArraysBuilder {
        DrawArraysBuilder::default()
            .gl(self.gl.clone())
            .program(program)
    }
    pub fn draw_elements(&self, program: Node<Program>) -> DrawElementsBuilder {
        DrawElementsBuilder::default()
            .gl(self.gl.clone())
            .program(program)
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


// pub fn vao(&self, attributes: impl Into<Attributes>) -> Result<VaoBuilder> {
//     let object = self
//         .gl
//         .create_vertex_array()
//         .ok_or(anyhow!("failed to create vertex array object"))?;
//     let builder = VaoBuilder::default()
//         .gl(self.gl.clone())
//         .object(object)
//         .attributes(attributes);
//     Ok(builder)
// }

// pub fn vertex_attribute(&self, buffer: impl Into<Buffer>) -> VertexAttributeBuilder {
//     VertexAttributeBuilder::default()
//         .gl(self.gl.clone())
//         .buffer(buffer)
// }
// pub fn bufferer(&self, buffer: impl Into<Buffer>) -> BuffererBuilder {
//     BuffererBuilder::default()
//         .gl(self.gl.clone())
//         .buffer(buffer)
// }
// pub fn buffer_in(&self, buffer: impl Into<Buffer>) -> BufferInBuilder {
//     BufferInBuilder::default()
//         .gl(self.gl.clone())
//         .buffer(buffer)
// }
