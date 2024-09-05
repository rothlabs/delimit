use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Element {
    html_element: u8,
    open: Hub,
    items: Vec<Hub>,
    close: Option<Hub>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(mut self, open: impl Into<Hub>) -> Self {
        self.open = open.into();
        self
    }
    pub fn item(mut self, item: impl Into<Hub>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn close(mut self) -> Result<Self> {
        let name = self.open.get("name")?;
        self.close = Some(name);
        Ok(self)
    }
    fn main(&self) -> Result<Gain> {
        let mut element = List::new()
            .separator("\n")
            .push(self.open.down(PLAIN)?)
            .extend(self.items.down(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.down(PLAIN)?)
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
        if let Some(close) = &mut self.close {
            close.deal("close", deal)?;
        }
        Ok(())
    }
}

impl Solve for Element {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Rank => 2.gain(),
            Task::Main => self.main(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}
