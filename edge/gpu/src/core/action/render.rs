use super::*;

pub struct Pass<'a> {
    pub core: &'a Core,
    pub command: &'a Command,
    pub descriptor: &'a RenderPassDescriptor<'a>,
}

impl<'a> Pass<'a> {
    pub fn compute(&self, encoder: &mut Encode<'_>) {
        let mut pass = encoder.compute();
        for cmd in &self.command.compute {
            match cmd {
                compute::Entry::Pipe(pipe) => pass.set_pipeline(pipe),
                compute::Entry::Bind(index, bind) => {
                    pass.set_bind_group(*index, &bind, &[])
                }
                compute::Entry::Dispatch(count) => {
                    pass.dispatch_workgroups(*count, 1, 1)
                }
            }
        }
    }
    pub fn render(&self, encoder: &mut Encode<'_>) {
        let mut pass = encoder.render(self.descriptor);
        for cmd in &self.command.render {
            match cmd {
                crate::render::Entry::Pipe(pipe) => pass.set_pipeline(pipe),
                crate::render::Entry::Bind(index, bind) => {
                    pass.set_bind_group(*index, &bind, &[])
                }
                crate::render::Entry::Vertex(slot, buffer) => {
                    pass.set_vertex_buffer(*slot, buffer.slice(..));
                }
                crate::render::Entry::Index(buffer) => {
                    pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
                }
                crate::render::Entry::Draw(vertices, instances) => {
                    pass.draw(vertices.clone(), instances.clone());
                }
                crate::render::Entry::DrawIndexed(indices, base_vertex, instances) => {
                    pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
                }
            }
        }
    }
}