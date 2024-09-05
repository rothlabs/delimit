use super::*;

#[derive(Default, Clone, Debug)]
pub struct Atlas;

impl DeserializeUnit for Atlas {
    fn deserialize(&self, serial: &Serial) -> graph::Result<Hub> {
        let unit: Unit = serde_json::from_str(&serial.unit)?;
        let imports = serial.imports.clone();
        Ok(unit.hub(imports))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Unit {
    Leaf(graph::work::Leaf),
    Bay(graph::Bay),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Unit {
    fn hub(self, imports: Vec<Import>) -> Hub {
        match self {
            Self::Leaf(x) => x.hub(),
            Self::Bay(x) => x.imports(imports).hub(),
            Self::TextPlainList(x) => x.imports(imports).hub(),
            Self::TextHtmlTag(x) => x.imports(imports).hub(),
            Self::TextHtmlAttribute(x) => x.imports(imports).hub(),
            Self::TextHtmlElement(x) => x.imports(imports).hub(),
        }
    }
}
