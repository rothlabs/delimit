use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Element {
    html_element: u8,
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
    async fn main(&self) -> Result<Gain<String>> {
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
        element.hub()?.gain()
    }
}

impl Adapt for Element {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.open.deal("open", deal)?;
        self.items.deal("items", deal)?;
        self.close.deal("close", deal)?;
        Ok(())
    }
}

impl Solve for Element {
    type Base = String;
    async fn solve(&self, task: Task<'_>) -> Result<Gain<String>> {
        match task {
            Task::Rank => 2.gain(),
            Task::Main => self.main().await,
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}
