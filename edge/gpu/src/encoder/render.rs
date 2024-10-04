use super::*;

pub struct Render<'a>(
    RenderPass<'a>,
);

impl<'a> Render<'a> {
    pub fn new(render_pass: RenderPass<'a>) -> Self {
        Self (render_pass)
    }
    pub fn pipeline(mut self, pipeline: &RenderPipeline) -> Self {
        self.0.set_pipeline(pipeline);
        self
    }
    pub fn debug(mut self, label: &str) -> Self {
        self.0.insert_debug_marker(label);
        self
    }
    pub fn draw(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.0.draw(vertices, instances);
        self
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct RenderAttachment<'a> {
    view: &'a TextureView,
    #[builder(default)]
    resolve_target: Option<&'a TextureView>
}

impl<'a> RenderAttachmentBuilder<'a> {
    pub fn make(self) -> Result<RenderPassColorAttachment<'a>> {
        let built = self.build()?;
        let state = RenderPassColorAttachment {
            view: built.view,
            resolve_target: built.resolve_target,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                store: wgpu::StoreOp::Store,
            },
        };
        Ok(state)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct RenderPassSetup<'a> {
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

impl<'a> RenderPassSetupBuilder<'a> {
    pub fn make(self) -> Result<RenderPassDescriptor<'a>> {
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