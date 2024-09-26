use super::*;

/// Vertex Array Object
/// Stores attribute settings and element array buffer target
#[derive(Builder, Debug, Make!)]
#[builder(pattern = "owned", build_fn(error = "graph::Error"), setter(into))]
pub struct VaoWriter {
    object: Vao,
    #[builder(setter(each(name = "attribute")))]
    attributes: Vec<Node<VertexAttribute>>,
    /// for ELEMENT_ARRAY_BUFFER only (ARRAY_BUFFER has no effect)
    #[builder(default)]
    index: Option<Buffer>,
}

impl Act for VaoWriter {
    async fn act(&self) -> Result<()> {
        self.object.bind();
        for attribute in &self.attributes {
            attribute.act().await?;
        }
        if let Some(buffer) = &self.index {
            buffer.bind();
        }
        self.object.unbind();
        Ok(())
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.attributes.back(back)
    }
}
