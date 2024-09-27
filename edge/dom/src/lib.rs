pub use text::*;

use web_sys::{Document, Element};
use derive_builder::Builder;
use graph::*;
use paste::paste;
pub use anyhow::anyhow;

mod text;

#[macro_use]
extern crate macro_rules_attribute;

struct Dom {
    pub document: Document,
}

impl Dom {
    pub fn text(&self) -> Result<TextBuilder> {
        let element = self.document.create_element("p")?;
        self.document.append_child(&element)?;
        Ok(TextBuilder::default().element(element))
    }
}