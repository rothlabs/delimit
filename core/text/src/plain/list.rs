use super::*;

#[derive(Serialize, Deserialize, Adapt, Digest, Default, Debug)]
pub struct List {
    plain_list: (),
    items: Vec<Hub<String>>,
    separator: Hub<String>,
}

impl List {
    pub fn hub(self) -> graph::Result<Hub<String>> {
        Ok(self.ploy()?.into())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_separator(&mut self, separator: impl Into<Hub<String>>) -> &mut Self {
        self.separator = separator.into();
        self
    }
    pub fn separator(mut self, separator: impl Into<Hub<String>>) -> Self {
        self.separator = separator.into();
        self
    }
    pub fn extend(mut self, items: Vec<impl Into<Hub<String>>>) -> Self {
        self.items.extend(items.into_iter().map(|item| item.into()));
        self
    }
    pub fn push(mut self, item: impl Into<Hub<String>>) -> Self {
        self.items.push(item.into());
        self
    }
    pub fn remove(&mut self, index: usize) -> &mut Self {
        self.items.remove(index);
        self
    }
}

impl Solve for List {
    type Base = String;
    async fn solve(&self) -> Result<Hub<String>> {
        if self.items.is_empty() {
            return solve_ok();
        }
        let last = self.items.len() - 1;
        let mut base = String::new();
        let separator = self.separator.base().await.unwrap_or_default();
        for i in 0..last {
            self.items[i].read(|x| base += x).await?;
            base += &separator;
        }
        self.items[last].read(|x| base += x).await?;
        Ok(base.into_leaf().hub())
    }
    fn rank(&self) -> u16 {
        PLAIN
    }
}