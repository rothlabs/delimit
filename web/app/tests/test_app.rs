//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

// extern crate wasm_bindgen_test;
use webapp::{demo, Result};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn demo_nurbs() -> Result<()> {
    demo::nurbs::DemoBuilder::default().make()?.start();
    Ok(())
}
