use super::*;

pub struct Elements {
    gl: WGLRC,
    program: Agent<Program>,
    buffer: Agent<Buffer<f32>>,
    vao: Agent<Vao>,
    count: Value<i32>,
    offset: Value<i32>,
}

impl Elements {
    pub fn link(
        gl: &WGLRC,
        program: &Agent<Program>,
        buffer: &Agent<Buffer<f32>>,
        vao: &Agent<Vao>,
    ) -> Agent<Elements> {
        Agent::make(|back| Self {
            gl: gl.clone(),
            program: program.backed(back),
            buffer: buffer.backed(back),
            vao: vao.backed(back),
            count: Value::default(),
            offset: Value::default(),
        })
    }
    pub fn count(&mut self, count: impl Into<Value<i32>>) -> &mut Self {
        self.count = count.into();
        self
    }
    pub fn offset(&mut self, offset: impl Into<Value<i32>>) -> &mut Self {
        self.offset = offset.into();
        self
    }
}

impl Act for Elements {
    type Load = react::Result;
    fn act(&self) -> Self::Load {
        let count = self.count.load();
        let offset = self.offset.load();
        self.program.act()?;
        self.program.read(|program| program.use_target());
        self.buffer.act();
        self.vao.act();
        self.vao.read(|vao| {
            vao.bind();
            self.gl
                .draw_elements_with_i32(WGLRC::TRIANGLES, count, WGLRC::UNSIGNED_SHORT, offset);
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
