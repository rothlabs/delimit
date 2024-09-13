use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into))]
pub struct DrawArrays {
    gl: WGLRC,
    program: Node<Program>,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    first: Hub<i32>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
    #[builder(default)]
    tfo: Option<Node<Tfo>>,
    #[builder(default)]
    rasterizer_discard: bool,
}

impl DrawArraysBuilder {
    pub fn make(&self) -> Result<Node<DrawArrays>> {
        let mut elements = self.build()?;
        Node::make(|back| {
            elements.program = elements.program.backed(back)?;
            elements.vao = elements.vao.backed(back)?;
            elements.first = elements.first.backed(back)?;
            elements.count = elements.count.backed(back)?;
            elements.tfo = elements.tfo.backed(back)?;
            Ok(elements)
        })
    }
}

impl DrawArrays {
    fn draw(&self, vao: &Vao) -> Result<()> {
        vao.bind();
        if self.rasterizer_discard {
            self.gl.enable(WGLRC::RASTERIZER_DISCARD);
        }
        if let Some(tfo) = &self.tfo {
            tfo.read(|tfo| {
                tfo.bind();
                self.gl.begin_transform_feedback(WGLRC::TRIANGLES);
                self.draw_triangles();
                self.gl.end_transform_feedback();
                tfo.unbind();
            })?;
        } else {
            self.draw_triangles();
        }
        if self.rasterizer_discard {
            self.gl.disable(WGLRC::RASTERIZER_DISCARD);
        }
        vao.unbind();
        Ok(())
    }
    fn draw_triangles(&self) {
        self.gl.draw_arrays(
            WGLRC::TRIANGLES,
            self.first.base().unwrap_or_default(),
            self.count.base().unwrap_or_default(),
        );
    }
}

impl Act for DrawArrays {
    fn act(&self) -> Result<()> {
        self.program.act()?;
        self.program.read(|unit| unit.use_())?;
        self.vao.act()?;
        self.vao.read(|vao| self.draw(vao))?
    }
}
