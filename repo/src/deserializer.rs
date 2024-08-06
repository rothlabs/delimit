use super::*;

#[derive(Debug, Clone)]
pub struct NodeDeserializer;

impl NodeDeserializer {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl DeserializeNode for NodeDeserializer {
    fn deserialize(&self, string: &str) -> Result<Node, Error> {
        let part: Part = serde_json::from_str(string)?;
        Ok(part.node())
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Part {
    GraphLoad(graph::Load),
    TextPlainList(text::plain::List),
    TextHtmlTag(text::html::Tag),
    TextHtmlAttribute(text::html::Attribute),
    TextHtmlElement(text::html::Element),
}

impl Part {
    fn node(&self) -> Node {
        match self {
            Self::GraphLoad(x) => x.leaf().node(),
            Self::TextPlainList(x) => x.node(),
            Self::TextHtmlTag(x) => x.node(),
            Self::TextHtmlAttribute(x) => x.node(),
            Self::TextHtmlElement(x) => x.node(),
        }
    }
}
