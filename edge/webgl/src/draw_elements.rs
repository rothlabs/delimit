use super::*;

#[derive(Builder, Debug, Gate)]
#[builder(pattern = "owned", setter(into))]
pub struct DrawElements {
    gl: WGLRC,
    #[builder(default, setter(each(name = "stem", into)))]
    stems: Vec<Apex>,
    program: Node<Program>,
    /// Vertex array object, collection of buffer attributes.
    vao: Vao,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    offset: Hub<i32>,
}

// impl GateTag for DrawElements {}

// impl DrawElementsBuilder
// where
//     DrawElements: Solve
// {
//     pub fn make(self) -> graph::Result<DrawElements> {
//         match self.build() {
//             Ok(value) => Ok(value),
//             Err(err) => Err(anyhow!(err.to_string()))?,
//         }
//     }
//     pub fn hub(self) -> graph::Result<Hub<<DrawElements as Solve>::Base>> {
//         Ok(self.make()?.gate()?.into())
//     }
// }

// impl IntoHub for Node<DrawArrays>
// where
//     DrawArrays: Solve
// {
//     type Base = DrawArrays::Base;
//     fn gate(self) -> Result<Gate<Self::Base>> {
//         Ok(self.as_gate()?.into())
//     }
// }
// where
//     #ident: Solve
// {
//     pub fn huby(self) -> graph::Result<Hub<<#ident as Solve>::Base>> {
//         Ok(self.as_gate()?.into())
//     }
// }

impl Act for DrawElements {
    async fn act(&self) -> Result<()> {
        self.stems.depend().await?;
        self.program.act().await?;
        self.program.read(|program| program.use_())?;
        let count = self.count.base().await.unwrap_or_default();
        let offset = self.offset.base().await.unwrap_or_default();
        self.vao.bind();
        self.gl
            .draw_elements_with_i32(WGLRC::TRIANGLES, count, WGLRC::UNSIGNED_SHORT, offset);
        self.vao.unbind();
        Ok(())
    }
}

impl Adapt for DrawElements {
    fn back(&mut self, back: &Back) -> Result<()> {
        self.stems.back(back)?;
        self.program.back(back)?;
        self.count.back(back)?;
        self.offset.back(back)
    }
}
