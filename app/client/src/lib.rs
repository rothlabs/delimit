use wasm_bindgen::prelude::*;
use web_sys::window;

// pub use draw::*;

pub mod draw;
pub mod gpu; 

#[wasm_bindgen(start)]
pub fn initialize() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let window = window().expect("no window");
    let _ = window.alert_with_message("Delimit!");
}

// alert("Hello, delimit!");

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
