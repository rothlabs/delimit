use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct DrawArrays {
    gl: WGLRC,
    program: Node<Program>,
    bufferers: Vec<Node<Bufferer>>,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    first: Hub<i32>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    #[builder(default)]
    tfo: Option<Tfo>,
    #[builder(default)]
    rasterizer_discard: bool,
}

impl DrawArraysBuilder {
    pub fn make(self) -> Result<Node<DrawArrays>> {
        let mut draw = self.build()?;
        Node::make(|back| {
            draw.program = draw.program.backed(back)?;
            draw.bufferers = draw.bufferers.backed(back)?;
            draw.vao = draw.vao.backed(back)?;
            draw.first = draw.first.backed(back)?;
            draw.count = draw.count.backed(back)?;
            Ok(draw)
        })
    }
}

impl DrawArrays {
    fn draw(&self, vao: &Vao, first: i32, count: i32) -> Result<()> {
        vao.bind();
        if self.rasterizer_discard {
            self.gl.enable(WGLRC::RASTERIZER_DISCARD);
        }
        if let Some(tfo) = &self.tfo {
            tfo.bind();
            self.gl.begin_transform_feedback(WGLRC::TRIANGLES);
            self.draw_triangles(first, count);
            self.gl.end_transform_feedback();
            tfo.unbind();
        } else {
            self.draw_triangles(first, count);
        }
        if self.rasterizer_discard {
            self.gl.disable(WGLRC::RASTERIZER_DISCARD);
        }
        vao.unbind();
        Ok(())
    }
    fn draw_triangles(&self, first: i32, count: i32) {
        self.gl.draw_arrays(WGLRC::TRIANGLES, first, count);
    }
}

impl Act for DrawArrays {
    async fn act(&self) -> Result<()> {
        self.program.act().await?;
        self.program.read(|unit| unit.use_())?;
        for bufferer in &self.bufferers {
            bufferer.act().await?;
        }
        self.vao.act().await?;
        let first = self.first.base().await.unwrap_or_default();
        let count = self.count.base().await.unwrap_or_default();
        self.vao.read(|vao| self.draw(vao, first, count))?
    }
}

impl Reckon for DrawArrays {}
