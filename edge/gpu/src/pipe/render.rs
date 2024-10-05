use super::*;
use std::num::NonZero;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Render<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    fragment: Option<FragmentState<'a>>,
    #[builder(default)]
    primitive: PrimitiveState,
    #[builder(default)]
    depth_stencil: Option<DepthStencilState>,
    #[builder(default)]
    multisample: MultisampleState,
    #[builder(default)]
    multiview: Option<NonZero<u32>>,
    #[builder(default)]
    cache: Option<&'a PipelineCache>,
}

impl RenderBuilder<'_> {
    pub fn make(self) -> Result<RenderPipeline> {
        let built = self.build()?;
        let descriptor = RenderPipelineDescriptor {
            label: built.label,
            layout: built.layout,
            vertex: built.vertex,
            fragment: built.fragment,
            primitive: built.primitive,
            depth_stencil: built.depth_stencil,
            multisample: built.multisample,
            multiview: built.multiview,
            cache: built.cache,
        };
        let value = built.device.create_render_pipeline(&descriptor);
        Ok(value)
    }
}

//#[builder(default = "PrimitiveState::default()")]
//#[builder(default = "MultisampleState::default()")]
