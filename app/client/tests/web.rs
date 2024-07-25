//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

mod gpu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_vertex_shader() {
    gpu::make_vertex_shader()
}

#[wasm_bindgen_test]
fn make_fragment_shader() {
    gpu::make_fragment_shader();
}

#[wasm_bindgen_test]
fn make_program() {
    gpu::make_program();
}

#[wasm_bindgen_test]
fn make_array_buffer() -> Result<(), String> {
    gpu::make_array_buffer()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_element_buffer() {
    gpu::make_element_buffer();
}

#[wasm_bindgen_test]
fn make_vertex_attribute() -> Result<(), String> {
    gpu::make_vertex_attribute()?;
    Ok(())
}

#[wasm_bindgen_test]
fn make_vertex_array_object() -> Result<(), String> {
    gpu::make_vertex_array_object()?;
    Ok(())
}

#[wasm_bindgen_test]
fn draw_elements() -> Result<(), String> {
    gpu::draw_elements()?;
    Ok(())
}

#[wasm_bindgen_test]
fn elements_react_to_shader_source() -> Result<(), String> {
    gpu::elements_react_to_shader_source()?;
    Ok(())
}

#[wasm_bindgen_test]
fn shader_source_error() -> Result<(), String> {
    gpu::shader_source_error()?;
    Ok(())
}
