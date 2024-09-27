use super::*;

#[derive(Builder, Debug, Unit!)]
#[builder(pattern = "owned", setter(into))]
pub struct Text {
    pub element: Element,
    pub content: Hub<String>,
}

impl Act for Text {
    async fn act(&self) -> Result<()> {
        let content = self.content.base().await.unwrap_or_default();
        self.element.set_text_content(Some(&content));
        Ok(())
    }
}