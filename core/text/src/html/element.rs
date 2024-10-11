use super::*;

#[derive(Default, Serialize, Deserialize, Debug, Adapt, Digest)]
pub struct Element {
    html_element: (),
    open: Hub<String>,
    items: Vec<Hub<String>>,
    close: Option<Hub<String>>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(mut self, open: impl Into<Hub<String>>) -> Self {
        self.open = open.into();
        self
    }
    pub fn item(mut self, item: impl Into<Hub<String>>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn close(mut self) -> Result<Self> {
        let name = self.open.get("name")?.string()?;
        self.close = Some(name);
        Ok(self)
    }
}

impl Solve for Element {
    type Base = String;
    async fn solve(&self) -> Result<Hub<String>> {
        let mut element = List::new()
            .separator("\n")
            .push(self.open.down(PLAIN).await?)
            .extend(self.items.down(PLAIN).await?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.down(PLAIN).await?)
                .push(">")
                .hub()?;
            element = element.push(close);
        }
        element.hub()
    }
    fn rank(&self) -> u16 {
        2
    }
}