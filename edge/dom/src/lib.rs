pub use text::*;

use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{js_sys::Object, window, Document, HtmlCanvasElement, HtmlElement};
use derive_builder::Builder;
use graph::*;
use paste::paste;
use anyhow::anyhow;
use gpu::*;

mod text;
mod canvas;

#[macro_use]
extern crate macro_rules_attribute;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error("JsValue Error ({0})")]
    JsValue(String),
    #[error("Object Error ({0})")]
    Object(String),
    #[error("Element Error ({0})")]
    Element(String),
    #[error("HtmlElement Error ({0})")]
    HtmlElement(String),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(format!("{:?}", value))
    }
}

impl From<Object> for Error {
    fn from(value: Object) -> Self {
        Error::Object(format!("{:?}", value))
    }
}

impl From<web_sys::Element> for Error {
    fn from(value: web_sys::Element) -> Self {
        Error::JsValue(format!("{:?}", value))
    }
}

impl From<web_sys::HtmlElement> for Error {
    fn from(value: web_sys::HtmlElement) -> Self {
        Error::JsValue(format!("{:?}", value))
    }
}

fn no_object() -> Error {
    Error::Object("none".into())
}

/// Dom Result
pub type Result<T> = std::result::Result<T, Error>;


pub struct Element {
    pub document: Document,
    pub object: HtmlElement,
}

impl Element {
    pub fn body() -> Result<Element> {
        let document = window().ok_or(anyhow!("no window"))?.document().ok_or(anyhow!("no document"))?;
        let object = document.body().ok_or(anyhow!("no body"))?;
        Ok(Element { document, object })
    }
    pub fn canvas(&self) -> Result<canvas::Canvas> {
        let object = self.object.clone().dyn_into::<HtmlCanvasElement>()?;
        Ok(canvas::Canvas{object})
    }
    pub fn element(&self, name: &str) -> Result<Element> {
        let object = self.object(name)?;
        self.down(object)
    }
    pub fn stem(&self, name: &str) -> Result<Element> {
        let object = self.object(name)?;
        self.object.append_child(&object)?;
        self.down(object)
    }
    fn object(&self, name: &str) -> Result<HtmlElement> {
        let object = self.document.create_element(name)?.dyn_into::<HtmlElement>()?;
        Ok(object)
    }
    fn down(&self, object: HtmlElement) -> Result<Element> {
        Ok(Element {document: self.document.clone(), object})
    }
}