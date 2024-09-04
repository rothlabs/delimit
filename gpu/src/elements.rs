use super::*;

/// Draw elements based on program, buffer, vertex array object (vao), count, and offset.
#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct Elements {
    gl: WGLRC,
    /// WebGL program
    program: Node<Program>,
    buffer: Node<Buffer>, // f32
    /// Vertex array object, collection of buffer attributes.
    vao: Node<Vao>,
    /// Number of values to draw.
    #[builder(default)]
    count: Apex,
    /// Number of values to skip before drawing.
    #[builder(default)]
    offset: Apex,
}

impl ElementsBuilder {
    pub fn make(&self) -> Result<Node<Elements>> {
        let mut elements = self.build().map_err(|err| anyhow!("{}", err.to_string()))?;
        Node::make(|back| {
            elements.program = elements.program.backed(back)?;
            elements.buffer = elements.buffer.backed(back)?;
            elements.vao = elements.vao.backed(back)?;
            elements.count = elements.count.backed(back)?;
            elements.offset = elements.offset.backed(back)?;
            Ok(elements)
        })
    }
}

impl Elements {
    fn draw(&self, vao: &Vao) {
        vao.bind();
        self.gl.draw_elements_with_i32(
            WGLRC::TRIANGLES,
            self.count.i32().unwrap_or_default(),
            WGLRC::UNSIGNED_SHORT,
            self.offset.i32().unwrap_or_default(),
        );
        vao.unbind();
    }
}

impl Act for Elements {
    fn act(&self) -> Result<()> {
        self.program.act()?;
        self.program.read(|program| program.use_())?;
        self.buffer.act()?;
        self.vao.act()?;
        self.vao.read(|vao| self.draw(vao))
    }
}
