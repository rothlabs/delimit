use super::*;

/// Draw elements based on program, buffer, vertex array object (vao), count, and offset.
#[derive(Builder)]
#[builder(setter(into))]
pub struct Elements {
    gl: WGLRC,
    /// WebGL program
    program: Agent<Program>,
    buffer: Agent<Buffer>, // f32
    /// Vertex array object, collection of buffer attributes.
    vao: Agent<Vao>,
    /// Number of values to draw.
    #[builder(default)]
    count: Node,
    /// Number of values to skip before drawing.
    #[builder(default)]
    offset: Node,
}

impl ElementsBuilder {
    pub fn link(&self) -> result::Result<Agent<Elements>, ElementsBuilderError> {
        let mut elements = self.build()?;
        let link = Agent::make(|back| {
            elements.program = elements.program.backed(back);
            elements.buffer = elements.buffer.backed(back);
            elements.vao = elements.vao.backed(back);
            elements.count = elements.count.backed(back);
            elements.offset = elements.offset.backed(back);
            elements
        });
        Ok(link)
    }
}

impl Solve for Elements {
    fn solve(&self, _: Task) -> solve::Result {
        self.program.solve(Task::Main)?;
        self.program.read(|program| program.use_());
        self.buffer.solve(Task::Main)?;
        self.vao.solve(Task::Main)?;
        self.vao.read(|vao| {
            vao.bind();
            self.gl.draw_elements_with_i32(
                WGLRC::TRIANGLES,
                self.count.i32(),
                WGLRC::UNSIGNED_SHORT,
                self.offset.i32(),
            );
            vao.unbind();
        });
        Ok(Tray::None)
    }
}