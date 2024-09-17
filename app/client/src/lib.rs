#[wasm_bindgen]
extern "C" {
    //pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use gloo_timers::future::TimeoutFuture;
pub use particle::*;

use wasm_bindgen::prelude::*;
use web_sys::window;
use prelude::*;
use gloo_render::request_animation_frame;

mod prelude {
    pub use graph::*;
    pub use gpu::*;
    pub use derive_builder::Builder;
    use wasm_bindgen::JsCast;
    use web_sys::{js_sys::Math::random, window, HtmlCanvasElement};
    pub struct Sim {
        pub gpu: Gpu,
    } 
    impl Sim {
        pub fn new() -> Self {
            let canvas = window().unwrap().document().unwrap().get_element_by_id("canvas").unwrap().dyn_into::<HtmlCanvasElement>().unwrap();
            canvas.set_width(1000);
            canvas.set_height(800);
            Self { gpu: Gpu { gl: canvas.get_context("webgl2") // canvas.get_context_with_context_options("webgl2", {preserveDrawingBuffer: true})
            .unwrap()
            .unwrap()
            .dyn_into::<WGLRC>()
            .unwrap() }}
        }
    }
    pub fn random_float() -> f32 {
        random() as f32 * 2. - 1.
    }
}
mod particle;


#[wasm_bindgen(start)]
pub async fn initialize() -> std::result::Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let sim = Sim::new();
    let tick = 0_i32.leaf();
    let particles = sim.particles(&tick).await.unwrap().make().unwrap();
    particles.act().await.unwrap();
    for _ in 0..1000 {
        tick.write(|x| *x += 1).await.unwrap();
        TimeoutFuture::new(16).await;
    }
    Ok(())
}

// gloo_render::request_animation_frame(|x| {
        
// });




// let window = window().expect("no window");
    // window.alert_with_message("Delimit!").ok();

// alert("Hello, delimit!");

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
