//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

mod gpu;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_vertex_shader() {
    gpu::make_vertex_shader();
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
fn draw_rect() {
    gpu::rect();
}
