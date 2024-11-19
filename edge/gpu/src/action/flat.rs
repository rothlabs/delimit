use super::*;

pub mod render;

#[derive(Clone, Debug)]
pub enum Command {
    Compute,
    Render(render::Pass),
}

// #[derive(Debug, Back, Gate)] // #[builder(pattern = "owned")]
// pub struct Buffer {
//     #[back(skip)]
//     viewport: Viewport,
//     commands: Hub<Vec<flat::Command>>,
// }

// impl Solve for Buffer {
//     type Base = Grc<CommandBuffer>;
//     async fn solve(&self) -> node::Result<Self::Base> {
//         let mut encoder = self.viewport.gpu.encoder();
//         let frame = self.viewport.frame()?;
//         let view = &frame.texture.create_view(&TextureViewDescriptor::default());
//         for command in &self.commands.base().await? {
//             if let flat::Command::Render(pass) = command {
//                 let attachments = self
//                     .viewport
//                     .gpu
//                     .attachment(&self.viewport.stage)
//                     .resolve_target(view)
//                     .list()?;
//                 let fields = &self.viewport.gpu.render_pass(&attachments).make()?;
//                 encoder.render(pass, fields);
//             }
//         }
//         Ok(Grc::new(encoder.finish()).into())
//     }
// }