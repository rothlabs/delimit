use super::*;

#[derive(Default, Serialize, Deserialize, Debug, GraphHash!)]
pub struct Attribute {
    html_attribute: u8,
    name: Hub<String>,
    content: Hub<String>,
}

impl Attribute {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<Hub<String>>) -> Self {
        self.name = name.into();
        self
    }
    pub fn content(mut self, content: impl Into<Hub<String>>) -> Self {
        self.content = content.into();
        self
    }
}

impl Solve for Attribute {
    type Base = String;
    async fn solve(&self) -> Result<Hub<String>> {
        List::new()
            .push(self.name.down(PLAIN).await?)
            .push(r#"=""#)
            .push(self.content.down(PLAIN).await?)
            .push(r#"""#)
            .hub()
    }
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.name.deal("name", deal)?;
        self.content.deal("content", deal)?;
        Ok(())
    }
    // fn reckon(&self, task: Task) -> Result<Gain> {
    //     match task {
    //         Task::Rank => 2.gain(),
    //         Task::Serial => self.serial(),
    //         Task::Digest(state) => self.digest(state),
    //         _ => task.no_handler(self),
    //     }
    // }
    fn rank(&self) -> u16 {
        2
    }
}

pub const ID: &str = "id";
pub const LANG: &str = "lang";
pub const CHARSET: &str = "charset";
pub const NAME: &str = "name";
pub const CONTENT: &str = "content";
pub const TYPE: &str = "type";
pub const SRC: &str = "src";

pub const ATTRIBUTES: [&str; 7] = [ID, LANG, CHARSET, NAME, CONTENT, TYPE, SRC];
