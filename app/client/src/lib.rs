use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn initialize() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    alert("Hello, delimit!");
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
