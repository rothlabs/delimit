pub use text::*;

use derive_builder::Builder;
use gpu::*;
use graph::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    js_sys::{Object, Promise},
    window, HtmlCanvasElement, HtmlElement,
};
use webgl::*;

mod canvas;
mod text;
// #[cfg(test)]
// mod tests;

#[macro_use]
extern crate macro_rules_attribute;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    Gpu(#[from] gpu::Error),
    #[error("JsValue Error ({0})")]
    JsValue(String),
    #[error("Object Error ({0})")]
    Object(String),
    #[error("Element Error ({0})")]
    Element(String),
    #[error("HtmlElement Error ({0})")]
    HtmlElement(String),
    #[error(transparent)]
    Any(#[from] anyError),
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

fn no_html_element() -> Error {
    Error::HtmlElement("none".into())
}

/// Dom Result
pub type Result<T> = std::result::Result<T, Error>;

pub struct Window {
    object: web_sys::Window,
}

impl Window {
    pub fn new() -> Result<Self> {
        let object = window().ok_or(anyhow!("no window"))?;
        Ok(Self { object })
    }
    pub fn document(&self) -> Result<Document> {
        let object = self.object.document().ok_or(anyhow!("no document"))?;
        Ok(Document { object })
    }
    pub fn request_animation_frame(&self) -> JsFuture {
        let promise = Promise::new(&mut |resolve, _| {
            self.object.request_animation_frame(&resolve).unwrap();
        });
        JsFuture::from(promise)
    }
}

#[derive(Clone)]
pub struct Document {
    object: web_sys::Document,
}

impl Document {
    pub fn body(&self) -> Result<Element> {
        let object = self.object.body().ok_or(no_html_element())?;
        Ok(Element {
            document: self.clone(),
            object,
        })
    }
    pub fn element(&self, name: &str) -> Result<HtmlElement> {
        let object = self
            .object
            .create_element(name)?
            .dyn_into::<HtmlElement>()?;
        Ok(object)
    }
    pub fn time(&self) -> Result<f64> {
        let time = self.object.timeline().current_time().result()?;
        Ok(time)
    }
}

pub struct Element {
    pub document: Document,
    pub object: HtmlElement,
}

impl Element {
    pub fn canvas(&self) -> Result<canvas::Canvas> {
        let object = self.object.clone().dyn_into::<HtmlCanvasElement>()?;
        Ok(canvas::Canvas { object })
    }
    pub fn element(&self, name: &str) -> Result<Self> {
        let object = self.document.element(name)?;
        self.down(object)
    }
    pub fn stem(&self, name: &str) -> Result<Self> {
        let object = self.document.element(name)?;
        self.object.append_child(&object)?;
        self.down(object)
    }
    fn down(&self, object: HtmlElement) -> Result<Self> {
        Ok(Self {
            document: self.document.clone(),
            object,
        })
    }
}

pub trait ToDomResult<T> {
    fn result(self) -> crate::Result<T>;
}

impl<T> ToDomResult<T> for Option<T> {
    fn result(self) -> crate::Result<T> {
        Ok(self.ok_or(anyhow!("option is none"))?)
    }
}
