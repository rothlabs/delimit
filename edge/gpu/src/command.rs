use super::*;

#[derive(Clone, Debug)]
pub enum Command {
    Compute,
    Render(render::Pass),
}

// #[derive(Builder, Gate, Back, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct Chain {
//     #[builder(default, setter(each(name = "pass", into)))]
//     passes: Vec<Pass>,
// }

// impl Solve for Chain {
//     type Base = Vec<Pass>;
//     async fn solve(&self) -> graph::Result<Hub<Vec<Pass>>> {
//         let passes = vec![];

//     }
// }

// impl Pass {
//     pub fn encode(&self, encoder: &mut Encode<'_>) {
//         let mut pass = encoder.compute();
//         for cmd in &self.pass.compute {
//             match cmd {
//                 compute::Entry::Pipe(pipe) => pass.set_pipeline(pipe),
//                 compute::Entry::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
//                 compute::Entry::Dispatch(count) => pass.dispatch_workgroups(*count, 1, 1),
//             }
//         }
//         let mut pass = encoder.render(self.descriptor);
//         for cmd in &self.pass.render {
//             match cmd {
//                 post::Render::Pipe(pipe) => pass.set_pipeline(pipe),
//                 post::Render::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
//                 post::Render::Vertex(slot, buffer) => {
//                     pass.set_vertex_buffer(*slot, buffer.slice(..));
//                 }
//                 post::Render::Index(buffer) => {
//                     pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
//                 }
//                 post::Render::Draw(vertices, instances) => {
//                     pass.draw(vertices.clone(), instances.clone());
//                 }
//                 post::Render::DrawIndexed(indices, base_vertex, instances) => {
//                     pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
//                 }
//             }
//         }
//     }
// }
