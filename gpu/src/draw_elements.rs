use super::*;

#[attr_alias::eval]
#[derive(Builder, Debug, Make!)]
#[attr_alias(build)]
pub struct DrawElements {
    gl: WGLRC,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    program: Node<Program>,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    offset: Hub<i32>,
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
        self.stems.poll().await?;
        self.program.act().await?;
        self.program.read(|program| program.use_())?;
        self.vao.act().await?;
        let count = self.count.base().await.unwrap_or_default();
        let offset = self.offset.base().await.unwrap_or_default();
        self.vao.read(|vao| self.draw(vao, count, offset))?;
        Ok(())
    }
    fn backed(&mut self, back: &Back) -> Result<()> {
        self.stems.back(back)?;
        self.program.back(back)?;
        self.vao.back(back)?;
        self.count.back(back)?;
        self.offset.back(back)
    }
}
