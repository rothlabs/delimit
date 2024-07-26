pub use buffer::Buffer;
pub use canvas::Canvas;
pub use elements::Elements;
pub use program::Program;
pub use shader::Shader;
pub use vao::Vao;
pub use vertex_attribute::VertexAttribute;

use buffer::Array;
use derive_builder::Builder;
use graph::*;
use js_sys::*;
use shader::*;
use std::{error::Error, result};
use texture::*;
use vao::*;
use vertex_attribute::VertexAttributeBuilder;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub mod buffer;
pub mod program;
pub mod shader;
pub mod vao;
pub mod texture;

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
    pub fn index_buffer(&self, array: impl Into<Array<u16>>) -> buffer::Result<u16> {
        Buffer::link_u16(&self.gl, WGLRC::ELEMENT_ARRAY_BUFFER, &array.into())
    }
    pub fn vertex_attribute(&self, buffer: &Agent<Buffer<f32>>) -> VertexAttributeBuilder {
        VertexAttributeBuilder::default()
            .gl(self.gl.clone())
            .buffer(buffer.clone())
            .clone()
    }
    pub fn vao(&self, attributes: &Attributes) -> result::Result<VaoBuilder, Box<dyn Error>> {
        let object = self
            .gl
            .create_vertex_array()
            .ok_or("failed to create vertex array object")?;
        Ok(VaoBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .attributes(attributes.clone())
            .clone())
    }
    pub fn texture<T: Copy>(&self, array: impl Into<Array<T>>) -> result::Result<TextureBuilder<T>, Box<dyn Error>> {
        let texture = self.gl.create_texture().ok_or("failed to create texture")?;
        self.gl.bind_texture(WGLRC::TEXTURE_2D, Some(&texture));
        self.gl.tex_parameteri(
            WGLRC::TEXTURE_2D,
            WGLRC::TEXTURE_MIN_FILTER,
            WGLRC::NEAREST as i32,
        );
        self.gl.tex_parameteri(
            WGLRC::TEXTURE_2D,
            WGLRC::TEXTURE_MAG_FILTER,
            WGLRC::NEAREST as i32,
        );
        Ok(TextureBuilder::default().gl(self.gl.clone()).texture(texture).array(array).clone())
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


    // <F: FnOnce(&mut VertexAttribute)>
    // pub fn vertex_attribute(&self, buffer: &Agent<Buffer<f32>>) -> Agent<VertexAttribute> {
    //     VertexAttribute::link(&self.gl, buffer)
    // }
