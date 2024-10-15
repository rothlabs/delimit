pub use buffer::*;
pub use buffer::Buffer;
pub use buffer_reader::*;
pub use bufferer::*;
pub use draw_arrays::*;
pub use draw_elements::*;
pub use program::*;
pub use shader::*;
pub use texture::*;
pub use tfo::*;
pub use vao::*;
pub use vertex_attribute::*;

use derive_builder::Builder;
use graph::*;
use vao_writer::*;
use web_sys::{js_sys::*, WebGl2RenderingContext, WebGlBuffer};

pub mod shader;
pub mod buffer;

mod buffer_reader;
mod bufferer;
mod draw_arrays;
mod draw_elements;
mod program;
mod texture;
mod tfo;
mod vao;
mod vao_writer;
mod vertex_attribute;

#[macro_use]
extern crate macro_rules_attribute;

pub type WGLRC = WebGl2RenderingContext;

#[derive(Debug, Clone)]
pub struct WebGl {
    pub gl: WGLRC,
}

impl From<WGLRC> for WebGl {
    fn from(wglrc: WGLRC) -> Self {
        Self { gl: wglrc }
    }
}

impl WebGl {
    pub fn clear(&self) {
        self.gl.clear(WGLRC::COLOR_BUFFER_BIT);
    }
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
        vertex.read(|unit| self.gl.attach_shader(&object, &unit.object))?;
        fragment.read(|unit| self.gl.attach_shader(&object, &unit.object))?;
        Ok(ProgramBuilder::default()
            .gl(self.gl.clone())
            .object(object)
            .vertex(vertex)
            .fragment(fragment))
    }
    pub fn buffer(&self) -> Result<buffer::Buffer> {
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
    pub fn vao(&self) -> Result<Vao> {
        let object = self
            .gl
            .create_vertex_array()
            .ok_or(anyhow!("failed to create vertex array object"))?;
        let vao = Vao {
            gl: self.gl.clone(),
            object,
        };
        Ok(vao)
    }
    pub fn tfo(&self) -> Result<TfoBuilder> {
        let object = self
            .gl
            .create_transform_feedback()
            .ok_or(anyhow!("failed to create transform feedback object"))?;
        let tfo = TfoBuilder::default().gl(self.gl.clone()).object(object);
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
            .array(array))
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

// macro_rules! make {
//     ($Unit:ident) => (
//         impl paste!{[<$Unit "Builder">]} {
//             pub fn make(self) -> Result<Node<$Unit>> {
//                 self.build()?.node()
//             }
//         }
//     )
// }

// make!{DrawElements}

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
