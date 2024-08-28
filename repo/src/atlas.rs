use super::*;

#[derive(Default, Clone, Debug)]
pub struct Atlas;

impl DeserializeUnit for Atlas {
    fn deserialize(&self, serial: &SerialNode) -> GraphResult<Apex> {
        let part: Part = serde_json::from_str(&serial.unit)?;
        Ok(part.apex(serial.imports.clone()))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    Leaf(graph::work::Leaf),
    Bay(graph::Bay),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn apex(self, imports: Vec<Import>) -> Apex {
        match self {
            Self::Leaf(x) => x.apex(),
            Self::Bay(x) => x.imports(imports).apex(),
            Self::TextPlainList(x) => x.imports(imports).apex(),
            Self::TextHtmlTag(x) => x.imports(imports).apex(),
            Self::TextHtmlAttribute(x) => x.imports(imports).apex(),
            Self::TextHtmlElement(x) => x.imports(imports).apex(),
        }
    }
}
