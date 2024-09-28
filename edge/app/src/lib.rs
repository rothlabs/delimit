use gloo_timers::future::TimeoutFuture;
use thiserror::Error;
use wasm_bindgen::prelude::*;
use derive_builder::{Builder, UninitializedFieldError};
use gpu::*;
use graph::*;
use dom::*;
use demo::*;
use web_sys::{js_sys::Math::random, window, HtmlCanvasElement};
use paste::paste;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

mod demo;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error("Js Error ({0})")]
    Js(String),
    #[error("Dom Error ({0})")]
    Dom(String),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<dom::Error> for Error {
    fn from(value: dom::Error) -> Self {
        Error::Js(format!("{:?}", value))
    }
}

/// App Result
pub type Result<T> = std::result::Result<T, Error>;

// TODO: rename to main? change to local result type?
#[wasm_bindgen(start)]
pub async fn initialize() -> std::result::Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    let demo = Demo::new();
    let tick = 0_i32.leaf();
    let particles = demo.nurbs(&tick).await.unwrap().node().unwrap();
    particles.act().await.unwrap();
    for _ in 0..100 {
        tick.write(|x| *x += 1).await.unwrap();
        TimeoutFuture::new(16).await;
    }
    Ok(())
}

pub fn random_float() -> f32 {
    random() as f32 * 2. - 1.
}

#[wasm_bindgen]
extern "C" {
    //pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
extern crate macro_rules_attribute;

// gloo_render::request_animation_frame(|x| {

// });

// let window = window().expect("no window");
// window.alert_with_message("Delimit!").ok();

// alert("Hello, delimit!");

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }
