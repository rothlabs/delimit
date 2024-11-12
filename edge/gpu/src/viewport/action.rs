use super::*;

mod render;

pub struct Render<'a> {
    pub display: &'a Viewport,
    pub chain: Vec<Command>,
}

impl<'a> Render<'a> {
    pub fn surface(&self) -> Result<()> {
        let mut encoder = self.display.core.encoder();
        let frame = self.display.frame()?;
        let view = &frame.texture.create_view(&TextureViewDescriptor::default());
        for command in &self.chain {
            let resolve_target = if command.msaa {
                panic!("not implemented");
                // self.core.attachment(view).resolve_target(target).list()?
            } else {
                // self.core.attachment(view).list()?
                None
            };
            let attachment = RenderPassColorAttachment {
                view,
                resolve_target,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                    store: wgpu::StoreOp::Store,
                },
            };
            let list = [Some(attachment); 1];
            let descriptor = &self.display.core.render_pass(&list).make()?;
            // let descriptor = &RenderPassDescriptor {
            //     label: Some("app_render"),
            //     color_attachments: &[Some(attachment); 1],
            //     depth_stencil_attachment: None,
            //     timestamp_writes: None,
            //     occlusion_query_set: None,
            // };
            let pass = render::Pass {
                command,
                descriptor,
            };
            if !command.compute.is_empty() {
                pass.compute(&mut encoder);
            }
            if !command.render.is_empty() {
                pass.render(&mut encoder);
            }
        }
        encoder.submit();
        frame.present();
        Ok(())
    }
}

// async fn compute_pass(&self, encoder: &mut Encode<'_>) -> graph::Result<()> {
//     let mut pass = encoder.compute();
//     for cmd in &self.compute_commands {
//         match cmd {
//             ComputeCommand::Pipe(pipe) => pass.set_pipeline(pipe),
//             ComputeCommand::Bind(index, bind) => {
//                 let bind = bind.base().await?;
//                 pass.set_bind_group(*index, &bind, &[])
//             }
//             ComputeCommand::Dispatch(count) => {
//                 let count = count.base().await?;
//                 pass.dispatch_workgroups(count, 1, 1)
//             }
//         }
//     }
//     Ok(())
// }
// async fn render_pass(&self, encoder: &mut Encode<'_>, view: &TextureView) -> graph::Result<()> {
//     let attachments = if let Some(target) = &self.resolve_target {
//         self.core.attachment(view).resolve_target(target).list()?
//     } else {
//         self.core.attachment(view).list()?
//     };
//     let render = self.core.render_pass(&attachments).make()?;
//     let mut pass = encoder.render(&render);
//     for cmd in &self.render_commands {
//         match cmd {
//             RenderCommand::Pipe(pipe) => pass.set_pipeline(pipe),
//             RenderCommand::Bind(index, bind) => {
//                 let bind = bind.base().await?;
//                 pass.set_bind_group(*index, &bind, &[])
//             }
//             RenderCommand::Vertex(slot, buffer) => {
//                 let buffer = buffer.base().await?;
//                 pass.set_vertex_buffer(*slot, buffer.slice(..));
//             }
//             RenderCommand::Index(buffer) => {
//                 let buffer = buffer.base().await?;
//                 pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
//             }
//             RenderCommand::Draw(vertices, instances) => {
//                 pass.draw(vertices.clone(), instances.clone());
//             }
//             RenderCommand::DrawIndexed((indices, base_vertex, instances)) => {
//                 pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
//             }
//         }
//     }
//     Ok(())
// }
// }

// impl Solve for Command {
// type Base = Mutation;
// async fn solve(&self) -> graph::Result<Hub<Mutation>> {
//     self.roots.depend().await?;
//     let mut encoder = self.core.encoder();
//     if !self.compute_commands.is_empty() {
//         self.compute_pass(&mut encoder).await?;
//     }
//     if let Some(display) = &self.display {
//         let view = &display.view();
//         let attachments = self.core.attachment(view).list()?;
//         let render = self.core.render_pass(&attachments).make()?;
//         {
//             let mut pass = encoder.render(&render);
//             for cmd in &self.render_commands {
//                 match cmd {
//                     RenderCommand::Pipe(pipe) => pass.set_pipeline(pipe),
//                     _ => ()
//                 }
//             }
//             pass.draw(0..3, 0..1);
//         }
//         //self.render_pass(&mut encoder, view).await?;
//     }
//     // if let Some(view) = &self.texture_view {
//     //     self.render_pass(&mut encoder, view).await?;
//     // }
//     println!("before gpu command submit");
//     encoder.submit();
//     println!("after gpu command submit");
//     Ok(Mutation.into())
// }
