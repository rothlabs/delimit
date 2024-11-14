use super::*;

pub struct Pass<'a> {
    pub command: &'a Command,
    pub descriptor: &'a RenderPassDescriptor<'a>,
}

impl<'a> Pass<'a> {
    pub fn compute(&self, encoder: &mut Encode<'_>) {
        let mut pass = encoder.compute();
        for cmd in &self.command.compute {
            match cmd {
                compute::Entry::Pipe(pipe) => pass.set_pipeline(pipe),
                compute::Entry::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
                compute::Entry::Dispatch(count) => pass.dispatch_workgroups(*count, 1, 1),
            }
        }
    }
    pub fn render(&self, encoder: &mut Encode<'_>) {
        let mut pass = encoder.render(self.descriptor);
        for cmd in &self.command.render {
            match cmd {
                post::Render::Pipe(pipe) => pass.set_pipeline(pipe),
                post::Render::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
                post::Render::Vertex(slot, buffer) => {
                    pass.set_vertex_buffer(*slot, buffer.slice(..));
                }
                post::Render::Index(buffer) => {
                    pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                post::Render::Draw(vertices, instances) => {
                    pass.draw(vertices.clone(), instances.clone());
                }
                post::Render::DrawIndexed(indices, base_vertex, instances) => {
                    pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }
    }
}
