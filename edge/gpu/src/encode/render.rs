use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Render<'a> {
    #[builder(default)]
    label: Option<&'a str>,
    attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    #[builder(default)]
    depth_stencil: Option<RenderPassDepthStencilAttachment<'a>>,
    #[builder(default)]
    timestamps: Option<RenderPassTimestampWrites<'a>>,
    #[builder(default)]
    occlusion_query: Option<&'a QuerySet>,
}

impl<'a> RenderBuilder<'a> {
    pub fn make(self) -> graph::Result<RenderPassDescriptor<'a>> {
        let built = self.build()?;
        let descriptor = RenderPassDescriptor {
            label: built.label,
            color_attachments: built.attachments,
            depth_stencil_attachment: built.depth_stencil,
            timestamp_writes: built.timestamps,
            occlusion_query_set: built.occlusion_query,
        };
        Ok(descriptor)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct ColorAttachment<'a> {
    view: &'a TextureView,
    #[builder(default)]
    resolve_target: Option<&'a TextureView>,
}

impl<'a> ColorAttachmentBuilder<'a> {
    pub fn make(self) -> graph::Result<RenderPassColorAttachment<'a>> {
        let built = self.build()?;
        let state = RenderPassColorAttachment {
            view: built.view,
            resolve_target: built.resolve_target,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                store: wgpu::StoreOp::Store,
            },
        };
        Ok(state)
    }
    pub fn list(self) -> graph::Result<[Option<RenderPassColorAttachment<'a>>; 1]> {
        Ok([Some(self.make()?); 1])
    }
}

// impl<'a> AttachmentListFrom<'a> for RenderPassColorAttachment<'a> {
//     fn list(self) -> [Option<RenderPassColorAttachment<'a>>; 1] {
//         [Some(self); 1]
//     }
// }
