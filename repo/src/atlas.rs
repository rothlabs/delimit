use super::*;

#[derive(Debug, Clone)]
pub struct Atlas;

impl Atlas {
    #[allow(dead_code)]
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl DeserializeApex for Atlas {
    fn deserialize(&self, string: &str) -> Result<Apex, Error> {
        let part: Part = serde_json::from_str(string)?;
        Ok(part.apex())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    GraphTray(graph::Tray),
    GraphBay(graph::Bay),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn apex(self) -> Apex {
        match self {
            Self::GraphTray(x) => x.into(),
            Self::GraphBay(x) => x.apex(),
            Self::TextPlainList(x) => x.apex(),
            Self::TextHtmlTag(x) => x.apex(),
            Self::TextHtmlAttribute(x) => x.apex(),
            Self::TextHtmlElement(x) => x.apex(),
        }
    }
}