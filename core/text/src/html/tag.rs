use super::*;

#[derive(Default, Serialize, Deserialize, Adapt, Digest, Debug)]
pub struct Tag {
    html_tag: (),
    name: Hub<String>,
    attributes: Vec<Hub<String>>,
}

impl Tag {
    pub fn hub(self) -> graph::Result<Hub<String>> {
        Ok(self.ploy()?.into())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<Hub<String>>) -> Self {
        self.name = name.into();
        self
    }
    pub fn attribute(mut self, attribute: impl Into<Hub<String>>) -> Self {
        self.attributes.push(attribute.into());
        self
    }
}

impl Solve for Tag {
    type Base = String;
    async fn solve(&self) -> Result<Hub<String>> {
        let items = List::new()
            .separator(" ")
            .push(self.name.down(PLAIN).await?)
            .extend(self.attributes.down(PLAIN).await?)
            .hub()?;
        List::new().push("<").push(&items).push(">").hub()
    }
    fn rank(&self) -> u16 {
        2
    }
}

pub const DOCTYPE: &str = "!DOCTYPE html";
pub const HTML: &str = "html";
pub const HEAD: &str = "head";
pub const TITLE: &str = "title";
pub const META: &str = "meta";
pub const SCRIPT: &str = "script";
pub const BODY: &str = "body";
pub const DIV: &str = "div";
pub const CANVAS: &str = "canvas";
pub const H1: &str = "h1";

pub const TAGS: [&str; 10] = [
    DOCTYPE, HTML, HEAD, TITLE, META, SCRIPT, BODY, DIV, CANVAS, H1,
];
