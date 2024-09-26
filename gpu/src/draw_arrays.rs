use super::*;

/// Draw arrays with WebGL.
#[derive(Builder, Debug, Make!)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct DrawArrays {
    gl: WGLRC,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    program: Node<Program>,
    #[builder(default = "WGLRC::TRIANGLES")]
    pub mode: u32,
    /// Vertex array object, collection of buffer attributes.
    vao: Vao,
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
}

impl DrawArrays {
    async fn rasterize_switch(&self) -> Result<()> {
        if self.rasterizer_discard {
            self.gl.enable(WGLRC::RASTERIZER_DISCARD);
            self.draw().await;
            self.gl.disable(WGLRC::RASTERIZER_DISCARD);
        } else {
            self.draw().await;
        }
        Ok(())
    }
    async fn draw(&self) {
        let first = self.first.base().await.unwrap_or_default();
        let count = self.count.base().await.unwrap_or_default();
        let instances = self.instances.base().await.unwrap_or_default().max(1);
        self.gl.draw_arrays_instanced(self.mode, first, count, instances);
    }
}

impl Act for DrawArrays {
    async fn act(&self) -> Result<()> {
        self.stems.poll().await?;
        self.program.act().await?;
        self.program.read(|unit| unit.use_())?;
        self.vao.bind();
        if let Some(tfo) = &self.tfo {
            tfo.begin(self.mode);
            self.rasterize_switch().await?;
            tfo.end();
        } else {
            self.rasterize_switch().await?;
        }
        self.vao.unbind();
        Ok(())
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.stems.back(back)?;
        self.program.back(back)?;
        self.first.back(back)?;
        self.count.back(back)?;
        self.instances.back(back)
    }
}
