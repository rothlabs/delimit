use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Element {
    html_element: u8,
    open: Apex,
    items: Vec<Apex>,
    close: Option<Apex>,
}

impl Element {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(mut self, open: impl Into<Apex>) -> Self {
        self.open = open.into();
        self
    }
    pub fn item(mut self, item: impl Into<Apex>) -> Self {
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
                .apex();
            element = element.push(close);
        }
        element.apex().gain()
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
            Task::Main => self.main(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}
