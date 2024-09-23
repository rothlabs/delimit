use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct DrawElements {
    gl: WGLRC,
    program: Node<Program>,
    buffers: Vec<Node<Bufferer>>,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    offset: Hub<i32>,
}

impl DrawElementsBuilder {
    pub fn make(self) -> Result<Node<DrawElements>> {
        self.build()?.node()
    }
}

impl DrawElements {
    // TODO: Make it so this can be async!!!!
    fn draw(&self, vao: &Vao, count: i32, offset: i32) {
        vao.bind();
        self.gl
            .draw_elements_with_i32(WGLRC::TRIANGLES, count, WGLRC::UNSIGNED_SHORT, offset);
        vao.unbind();
    }
}

impl Act for DrawElements {
    async fn act(&self) -> Result<()> {
        self.program.act().await?;
        self.program.read(|program| program.use_())?;
        for buffer in &self.buffers {
            buffer.act().await?;
        }
        self.vao.act().await?;
        let count = self.count.base().await.unwrap_or_default();
        let offset = self.offset.base().await.unwrap_or_default();
        self.vao.read(|vao| self.draw(vao, count, offset))?;
        Ok(())
    }
    fn back(&mut self, back: &Back) -> Result<()> {
        self.program = self.program.backed(back)?;
        self.buffers = self.buffers.backed(back)?;
        self.vao = self.vao.backed(back)?;
        self.count = self.count.backed(back)?;
        self.offset = self.offset.backed(back)?;
        Ok(())
    }
}
