pub use anyhow::anyhow;
pub use buffer_data::BufferData;
pub use buffer::Buffer;
pub use buffer_out::BufferOut;
pub use canvas::Canvas;
pub use draw_elements::DrawElements;
pub use draw_arrays::DrawArrays;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use tfo::Tfo;
pub use vertex_attribute::VertexAttribute;

use derive_builder::Builder;
use draw_arrays::DrawArraysBuilder;
use tfo::TfoBuilder;
use program::ProgramBuilder;
use draw_elements::DrawElementsBuilder;
use graph::*;
use texture::*;
use vao::*;
use vertex_attribute::VertexAttributeBuilder;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::*, WebGl2RenderingContext, WebGlBuffer};

pub mod buffer;
pub mod buffer_data;
pub mod buffer_out;
pub mod program;
pub mod shader;
pub mod texture;
pub mod vao;
pub mod tfo;

mod canvas;
mod draw_elements;
mod draw_arrays;
mod vertex_attribute;

pub type WGLRC = WebGl2RenderingContext;

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
    pub fn program(&self, vertex: &Node<Shader>, fragment: &Node<Shader>) -> Result<ProgramBuilder> {
        let object = self
            .gl
            .create_program()
            .ok_or(anyhow!("failed to create program"))?;
        Ok(ProgramBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .vertex(vertex.clone())
            .fragment(fragment.clone())
            .clone())
    }
    pub fn array_buffer(&self) -> Result<Node<Buffer>> {
        let object = self
            .gl
            .create_buffer()
            .ok_or(anyhow!("failed to create buffer"))?;
        Node::make(|_| Ok(Buffer {
            gl: self.gl.clone(),
            object,
            target: WGLRC::ARRAY_BUFFER
        }))
    }
    pub fn index_buffer(&self) -> Result<Node<Buffer>> {
        let object = self
            .gl
            .create_buffer()
            .ok_or(anyhow!("failed to create buffer"))?;
        Node::make(|_| Ok(Buffer {
            gl: self.gl.clone(),
            object,
            target: WGLRC::ELEMENT_ARRAY_BUFFER
        }))
    }
    pub fn buffer(&self, buffer: &Node<Buffer>, array: impl Into<Apex>) -> Result<Node<BufferData>> {
        Node::make(|back| Ok(BufferData {
            gl: self.gl.clone(),
            buffer: buffer.backed(back)?,
            array: array.into().backed(back)?,
        }))
    }
    // pub fn feedback_buffer(&self, count: impl Into<Hub<i32>>) -> Result<Node<BufferOut>> {
    //     BufferOut::make(&self.gl, WGLRC::TRANSFORM_FEEDBACK_BUFFER, count.into())
    // }
    pub fn vertex_attribute(&self, buffer: &Node<Buffer>) -> VertexAttributeBuilder {
        VertexAttributeBuilder::default()
            .gl(self.gl.clone())
            .buffer(buffer.clone())
            .clone()
    }
    pub fn vao(&self, attributes: &Attributes) -> Result<VaoBuilder> {
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
    pub fn tfo(&self, buffer: &Node<BufferData>) -> Result<TfoBuilder> {
        let object = self
            .gl
            .create_transform_feedback()
            .ok_or(anyhow!("failed to create transform feedback object"))?;
        Ok(TfoBuilder::default().object(object).buffer(buffer.clone()).clone())
    }
    pub fn texture(
        &self,
        array: impl Into<Apex>,
    ) -> Result<TextureBuilder> {
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
    pub fn draw_arrays(&self, program: &Node<Program>) -> DrawArraysBuilder {
        DrawArraysBuilder::default()
            .gl(self.gl.clone())
            .program(program.clone())
            .clone()
    }
    pub fn draw_elements(&self, program: &Node<Program>) -> DrawElementsBuilder {
        DrawElementsBuilder::default()
            .gl(self.gl.clone())
            .program(program.clone())
            .clone()
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