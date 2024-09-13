use super::*;

/// Draw elements on WebGL canvas.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into), build_fn(error = "graph::Error"))]
pub struct DrawElements {
    gl: WGLRC,
    program: Node<Program>,
    buffers: Vec<Node<BufferData>>,
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
        let mut elements = self.build()?;
        Node::make(|back| {
            elements.program = elements.program.backed(back)?;
            elements.buffers = elements.buffers.backed(back)?;
            elements.vao = elements.vao.backed(back)?;
            elements.count = elements.count.backed(back)?;
            elements.offset = elements.offset.backed(back)?;
            Ok(elements)
        })
    }
}

impl DrawElements {
    fn draw(&self, vao: &Vao) {
        vao.bind();
        self.gl.draw_elements_with_i32(
            WGLRC::TRIANGLES,
            self.count.base().unwrap_or_default(),
            WGLRC::UNSIGNED_SHORT,
            self.offset.base().unwrap_or_default(),
        );
        vao.unbind();
    }
}

impl Act for DrawElements {
    fn act(&self) -> Result<()> {
        self.program.act()?;
        self.program.read(|program| program.use_())?;
        for buffer in &self.buffers {
            buffer.act()?;
        }
        self.vao.act()?;
        self.vao.read(|vao| self.draw(vao))
    }
}
