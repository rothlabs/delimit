use super::*;

/// Vertex Array Object
/// Stores attribute settings and element array buffer target
#[derive(Builder, Debug, BuildGate)]
#[builder(pattern = "owned", setter(into))]
pub struct VaoWriter {
    object: Vao,
    #[builder(setter(each(name = "attribute")))]
    attributes: Vec<Node<VertexAttribute>>,
    /// for ELEMENT_ARRAY_BUFFER only (ARRAY_BUFFER has no effect)
    #[builder(default)]
    index: Option<buffer::Buffer>,
}

impl Act for VaoWriter {
    async fn act(&self) -> node::Action {
        self.object.bind();
        for attribute in &self.attributes {
            attribute.act().await?;
        }
        if let Some(buffer) = &self.index {
            buffer.bind();
        }
        self.object.unbind();
        acted()
    }
}

impl Adapt for VaoWriter {
    fn back(&mut self, back: &Back) -> graph::Result<()> {
        self.attributes.back(back)
    }
}
