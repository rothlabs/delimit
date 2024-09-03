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
    pub fn close(mut self) -> Self {
        let name = self.open.get("name").expect("No opening tag name.");
        self.close = Some(name);
        self
    }
    fn set_at(&mut self, index: usize, apex: Apex) -> Result<Memo> {
        self.items[index] = apex;
        adapt_ok()
    }
    fn main(&self) -> Result<Gain> {
        let mut element = List::new()
            .separator("\n")
            .push(self.open.at(PLAIN)?)
            .extend(self.items.at(PLAIN)?);
        if let Some(close) = &self.close {
            let close = List::new()
                .push("</")
                .push(close.at(PLAIN)?)
                .push(">")
                .apex();
            element = element.push(close);
        }
        element.apex().gain()
    }
}

impl Adapt for Element {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<Memo> {
        self.open.deal("open", deal)?;
        self.items.deal("items", deal)?;
        if let Some(close) = &mut self.close {
            close.deal("close", deal)?;
        }
        adapt_ok()
    }
}

impl Solve for Element {
    fn solve(&self, task: Task) -> Result<Gain> {
        match task {
            Task::Main => self.main(),
            Task::Serial => self.serial(),
            Task::Digest(state) => self.digest(state),
            _ => task.no_handler(self),
        }
    }
}

// fn all(&self) -> Result<Gain> {
//     let mut apexes = vec![self.open.clone()];
//     apexes.extend(self.items.clone());
//     if let Some(apex) = &self.close {
//         apexes.push(apex.clone());
//     }
//     apexes.gain()
// }
