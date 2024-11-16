use super::*;

#[derive(Debug, Builder, BuildGate)]
#[builder(pattern = "owned", setter(into))]
pub struct Text {
    pub element: web_sys::Element,
    pub content: Hub<String>,
}

impl Act for Text {
    async fn act(&self) -> node::Action {
        let content = self.content.base().await.unwrap_or_default();
        self.element.set_text_content(Some(&content));
        acted()
    }
}

impl Adapt for Text {
    fn adapt(&mut self, deal: &mut dyn Deal) -> graph::Result<()> {
        self.content.deal("content", deal)
    }
}
