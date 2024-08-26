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
    fn deserialize(&self, string: &str) -> Result<Box<dyn EngageUnit>, Error> {
        let part: Part = serde_json::from_str(string)?;
        Ok(part.unit())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    // GraphTray(graph::Tray),
    // GraphLeaf(graph::work::Leaf),
    GraphBay(graph::Bay),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn unit(self) -> Box<dyn EngageUnit> {
        match self {
            // Self::GraphTray(x) => x.into(),
            // Self::GraphLeaf(x) => x.apex(),
            Self::GraphBay(x) => Box::new(x),
            Self::TextPlainList(x) => Box::new(x),
            Self::TextHtmlTag(x) => Box::new(x),
            Self::TextHtmlAttribute(x) => Box::new(x),
            Self::TextHtmlElement(x) => Box::new(x),
        }
    }
}
