use super::*;

#[derive(Debug, Clone)]
pub struct Atlas;

impl Atlas {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl DeserializeNode for Atlas {
    fn deserialize(&self, string: &str) -> node::Result {
        let part: Part = serde_json::from_str(string)?;
        Ok(part.node())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    GraphTray(graph::Tray),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn node(self) -> Node {
        match self {
            Self::GraphTray(x) => x.into(),
            Self::TextPlainList(x) => x.node(),
            Self::TextHtmlTag(x) => x.node(),
            Self::TextHtmlAttribute(x) => x.node(),
            Self::TextHtmlElement(x) => x.node(),
        }
    }
}
