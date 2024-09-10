use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(into))]
pub struct DrawArrays {
    gl: WGLRC,
    program: Node<Program>,
    buffer: Node<Buffer>,
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to skip before drawing.
    #[builder(default)]
    first: Hub<i32>,
    /// Number of values to draw.
    #[builder(default)]
    count: Hub<i32>,
}

impl DrawArraysBuilder {
    pub fn make(&self) -> Result<Node<DrawArrays>> {
        let mut elements = self.build()?;
        Node::make(|back| {
            elements.program = elements.program.backed(back)?;
            elements.buffer = elements.buffer.backed(back)?;
            elements.vao = elements.vao.backed(back)?;
            elements.first = elements.first.backed(back)?;
            elements.count = elements.count.backed(back)?;
            Ok(elements)
        })
    }
}

impl DrawArrays {
    fn draw(&self, vao: &Vao) {
        vao.bind();
        self.gl.draw_arrays(
            WGLRC::POINTS,
            self.first.base().unwrap_or_default(),
            self.count.base().unwrap_or_default(),
        );
        vao.unbind();
    }
}

impl Act for DrawArrays {
    fn act(&self) -> Result<()> {
        self.program.act()?;
        self.program.read(|unit| unit.use_())?;
        self.buffer.act()?;
        self.vao.act()?;
        self.vao.read(|vao| self.draw(vao))
    }
}
