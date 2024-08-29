use graph::*;
use serde::Serialize;

// use text::html::{attribute_set, Doc};

use crate::INIT;

#[cfg(test)]
mod tests;

#[derive(Default, Serialize)]
pub struct Importmap {
    imports: Imports,
}

#[derive(Serialize)]
pub struct Imports {
    init: String,
}

impl Default for Imports {
    fn default() -> Self {
        Self {
            init: String::from(INIT),
        }
    }
}

pub fn index() -> Result<String> {
    Ok("page".into())
}

//     let atts = attribute_set();
//     let mut html = Doc::new(&atts, "Delimit index page").html();
//     html.attribute("lang", "en");
//     let mut title = html.head().title();
//     title.add_str("Delimit");
//     let mut meta = title.root().meta();
//     meta.attribute("charset", "utf-8");
//     meta = meta.root().meta();
//     meta.attribute("name", "viewport")
//         .attribute("content", "width=device-width, initial-scale=1");
//     meta = meta.root().meta();
//     meta.attribute("name", "author")
//         .attribute("content", "Roth Labs LLC");
//     let mut script = meta.root().script();
//     script.attribute("type", "importmap");
//     script.add_str(&serde_json::to_string(&Importmap::default()).unwrap());
//     let mut body = script.up_to_html().body();
//     body.add_str("Delimit");
//     let mut canvas = body.canvas();
//     canvas.attribute("id", "canvas");
//     let mut script = canvas.root().script();
//     script.attribute("src", BOOT).attribute("type", "module");
//     let string = script.up_to_doc().string()?;
