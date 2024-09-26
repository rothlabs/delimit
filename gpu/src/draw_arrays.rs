use super::*;

/// Draw arrays with WebGL.
#[attr_alias::eval]
#[derive(Builder, Debug, Make!)]
#[attr_alias(build)]
pub struct DrawArrays {
    gl: WGLRC,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    program: Node<Program>,
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
        self.stems.poll().await?;
        self.program.act().await?;
        self.program.read(|unit| unit.use_())?;
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
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.stems.back(back)?;
        self.program.back(back)?;
        self.vao.back(back)?;
        self.first.back(back)?;
        self.count.back(back)?;
        self.instances.back(back)
    }
}
