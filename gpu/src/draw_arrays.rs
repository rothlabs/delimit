use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct DrawArrays {
    gl: WGLRC,
    program: Node<Program>,
    #[builder(default, setter(each(name = "writer")))]
    writers: Vec<Node<Bufferer>>,
    #[builder(default = "WGLRC::TRIANGLES")]
    pub mode: u32,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    first: Hub<i32>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    #[builder(default)]
    instances: Hub<i32>,
    #[builder(default)]
    tfo: Option<Tfo>,
    #[builder(default)]
    rasterizer_discard: bool,
    #[builder(default)]
    tick: Hub<i32>,
    // #[builder(default)]
    // step: Hub<i32>,
}

impl DrawArraysBuilder {
    pub fn make(self) -> Result<Node<DrawArrays>> {
        let mut draw = self.build()?;
        Node::make(|back| {
            draw.program = draw.program.backed(back)?;
            draw.writers = draw.writers.backed(back)?;
            draw.vao = draw.vao.backed(back)?;
            draw.first = draw.first.backed(back)?;
            draw.count = draw.count.backed(back)?;
            draw.instances = draw.instances.backed(back)?;
            draw.tick = draw.tick.backed(back)?;
            // draw.step = draw.step.backed(back)?;
            Ok(draw)
        })
    }
}

impl DrawArrays {
    fn draw(&self, vao: &Vao, first: i32, count: i32, instances: i32) -> Result<()> {
        vao.bind();
        if self.rasterizer_discard {
            self.gl.enable(WGLRC::RASTERIZER_DISCARD);
            // self.gl.draw_arrays(self.mode, first, count);
            self.gl
                .draw_arrays_instanced(self.mode, first, count, instances);
            self.gl.disable(WGLRC::RASTERIZER_DISCARD);
        } else {
            // self.gl.draw_arrays(self.mode, first, count);
            self.gl
                .draw_arrays_instanced(self.mode, first, count, instances);
        }
        vao.unbind();
        Ok(())
    }
}

impl Act for DrawArrays {
    async fn act(&self) -> Result<()> {
        self.tick.base().await.unwrap_or_default();
        self.program.act().await?;
        self.program.read(|unit| unit.use_())?;
        for bufferer in &self.writers {
            bufferer.act().await?;
        }
        self.vao.act().await?;
        let first = self.first.base().await.unwrap_or_default();
        let count = self.count.base().await.unwrap_or_default();
        let instances = self.instances.base().await.unwrap_or_default().max(1);
        if let Some(tfo) = &self.tfo {
            tfo.begin(self.mode);
            self.vao
                .read(|vao| self.draw(vao, first, count, instances))??;
            tfo.end();
        } else {
            self.vao
                .read(|vao| self.draw(vao, first, count, instances))??;
        }
        Ok(())
    }
}

impl Reckon for DrawArrays {}
