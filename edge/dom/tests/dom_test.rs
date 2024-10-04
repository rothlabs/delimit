//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use dom::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn make_canvas() -> Result<()> {
    Window::new()?
        .document()?
        .body()?
        .stem("canvas")?
        .canvas()?;
    Ok(())
}
