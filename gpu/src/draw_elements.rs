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
        let mut draw = self.build()?;
        Node::make(|back| {
            draw.program = draw.program.backed(back)?;
            draw.buffers = draw.buffers.backed(back)?;
            draw.vao = draw.vao.backed(back)?;
            draw.count = draw.count.backed(back)?;
            draw.offset = draw.offset.backed(back)?;
            Ok(draw)
        })
    }
}

impl DrawElements {
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
}

impl Reckon for DrawElements {}
