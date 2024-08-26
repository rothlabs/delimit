use super::*;

#[derive(Debug, Clone)]
pub struct Atlas;

impl Atlas {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl DeserializeUnit for Atlas {
    fn deserialize(&self, serial: &SerialNode) -> Result<Apex, Error> {
        let part: Part = serde_json::from_str(&serial.unit)?;
        Ok(part.apex(serial.imports.clone()))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    GraphTray(graph::Tray),
    GraphLeaf(graph::work::Leaf),
    GraphBay(graph::Bay),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn apex(self, imports: Vec<Import>) -> Apex {
        match self {
            Self::GraphTray(x) => x.into(),
            Self::GraphLeaf(x) => x.apex(),
            Self::GraphBay(x) => x.imports(imports).apex(),
            Self::TextPlainList(x) => x.imports(imports).apex(),
            Self::TextHtmlTag(x) => x.imports(imports).apex(),
            Self::TextHtmlAttribute(x) => x.imports(imports).apex(),
            Self::TextHtmlElement(x) => x.imports(imports).apex(),
        }
    }
}
