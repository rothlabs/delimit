pub use text::*;

use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Document, HtmlCanvasElement, HtmlElement};
use derive_builder::Builder;
use graph::*;
use paste::paste;
use anyhow::anyhow;

mod text;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error("Js Error ({0})")]
    Js(String),
    #[error("Element Error ({0})")]
    Element(String),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::Js(format!("{:?}", value))
    }
}

impl From<web_sys::Element> for Error {
    fn from(value: web_sys::Element) -> Self {
        Error::Js(format!("{:?}", value))
    }
}

/// Dom Result
pub type Result<T> = std::result::Result<T, Error>;

pub struct Element {
    pub document: Document,
    pub object: HtmlElement,
}

impl Element {
    // pub fn new(document: Document) -> Result<Self> {
    //     let object = document.body().ok_or(anyhow!("no body"))?;
    //     Ok(Self { document, object })
    // }
    pub fn text(&self) -> Result<TextBuilder> {
        let element = self.document.create_element("p")?;
        self.object.append_child(&element)?;
        Ok(TextBuilder::default().element(element))
    }
    pub fn canvas(&self) -> Result<HtmlCanvasElement> {
        let element = self.document.create_element("canvas")?.dyn_into::<HtmlCanvasElement>()?;
        Ok(element)
    }
}

#[macro_use]
extern crate macro_rules_attribute;