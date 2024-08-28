//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use graph::GraphResult;
use std::error::Error;
use wasm_bindgen_test::*;

mod gpu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_vertex_shader() -> GraphResult<()> {
    gpu::make_vertex_shader()
}

#[wasm_bindgen_test]
fn make_fragment_shader() -> GraphResult<()> {
    gpu::make_fragment_shader()
}

#[wasm_bindgen_test]
fn make_program() -> GraphResult<()> {
    gpu::make_program()
}

#[wasm_bindgen_test]
fn make_buffer() -> Result<(), Box<dyn Error>> {
    gpu::make_buffer()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_index_buffer() -> GraphResult<()> {
    gpu::make_index_buffer()
}

#[wasm_bindgen_test]
fn make_vertex_attribute() -> Result<(), Box<dyn Error>> {
    gpu::make_vertex_attribute()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_vertex_array_object() -> Result<(), Box<dyn Error>> {
    gpu::make_vertex_array_object()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements() -> Result<(), Box<dyn Error>> {
    gpu::draw_elements()?;
    Ok(())
}

#[wasm_bindgen_test]
fn elements_react_to_shader_source() -> Result<(), Box<dyn Error>> {
    gpu::elements_react_to_shader_source()?;
    Ok(())
}

#[wasm_bindgen_test]
fn shader_source_error() -> Result<(), Box<dyn Error>> {
    gpu::shader_source_error()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements_textured() -> Result<(), Box<dyn Error>> {
    gpu::draw_elements_textured()?;
    Ok(())
}
