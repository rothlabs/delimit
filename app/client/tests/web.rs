//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use graph::Result;
// use std::error::Error;
use wasm_bindgen_test::*;

mod gpu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_vertex_shader() -> Result<()> {
    gpu::make_vertex_shader()
}

#[wasm_bindgen_test]
fn make_fragment_shader() -> Result<()> {
    gpu::make_fragment_shader()
}

#[wasm_bindgen_test]
fn make_program() -> Result<()> {
    gpu::make_program()
}

#[wasm_bindgen_test]
fn make_buffer() -> Result<()> {
    gpu::make_buffer()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_index_buffer() -> Result<()> {
    gpu::make_index_buffer()
}

#[wasm_bindgen_test]
fn make_vertex_attribute() -> Result<()> {
    gpu::make_vertex_attribute()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_vertex_array_object() -> Result<()> {
    gpu::make_vertex_array_object()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements() -> Result<()> {
    gpu::draw_elements()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements_react_to_shader_source() -> Result<()> {
    gpu::draw_elements_react_to_shader_source()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements_react_to_buffer_array() -> Result<()> {
    gpu::draw_elements_react_to_buffer_array()?;
    Ok(())
}

#[wasm_bindgen_test]
fn shader_source_error() -> Result<()> {
    gpu::shader_source_error()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements_textured() -> Result<()> {
    gpu::draw_elements_textured()?;
    Ok(())
}
