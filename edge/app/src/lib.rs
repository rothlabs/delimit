use thiserror::Error;
use wasm_bindgen::prelude::*;
use derive_builder::{Builder, UninitializedFieldError};
use gpu::*;
use graph::*;
use dom::*;
use web_sys::js_sys::Math::random;
use paste::paste;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub mod demo;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error("Js Error ({0})")]
    JsValue(String),
    #[error("Dom Error ({0})")]
    Dom(String),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(format!("{:?}", value))
    }
}

impl From<dom::Error> for Error {
    fn from(value: dom::Error) -> Self {
        Error::Dom(format!("{:?}", value))
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

/// App Result
pub type Result<T> = std::result::Result<T, Error>;

#[wasm_bindgen(start)]
pub async fn entry() -> Result<()> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    demo::nurbs::DemoBuilder::default().duration(10000.).width(1200).height(900).make()?.run().await?;
    Ok(())
}

pub fn random_float() -> f32 {
    random() as f32 * 2. - 1.
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
extern crate macro_rules_attribute;

// pub trait ToResult<T> {
//     fn result(&self) -> Result<T> {
//         self.
//     }
// }