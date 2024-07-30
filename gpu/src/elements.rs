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

impl Act for Elements {
    type Load = react::Result;
    fn act(&self) -> Self::Load {
        self.program.act()?;
        self.program.read(|program| program.use_());
        self.buffer.act();
        self.vao.act();
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
        Ok(())
    }
}

impl React for Elements {
    fn react(&self, _: &Meta) -> react::Result {
        self.act()
    }
}

// pub struct Elements {
//     gl: WGLRC,
//     program: Agent<Program>,
//     buffer: Agent<Buffer<f32>>,
//     vao: Agent<Vao>,
//     count: Value<i32>,
//     offset: Value<i32>,
// }

// impl Elements {
//     pub fn link(
//         gl: &WGLRC,
//         program: &Agent<Program>,
//         buffer: &Agent<Buffer<f32>>,
//         vao: &Agent<Vao>,
//     ) -> Agent<Elements> {
//         Agent::make(|back| Self {
//             gl: gl.clone(),
//             program: program.backed(back),
//             buffer: buffer.backed(back),
//             vao: vao.backed(back),
//             count: Value::default(),
//             offset: Value::default(),
//         })
//     }
//     pub fn count(&mut self, count: impl Into<Value<i32>>) -> &mut Self {
//         self.count = count.into();
//         self
//     }
//     pub fn offset(&mut self, offset: impl Into<Value<i32>>) -> &mut Self {
//         self.offset = offset.into();
//         self
//     }
// }
