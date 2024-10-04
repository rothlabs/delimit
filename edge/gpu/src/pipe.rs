pub use shader::*;

use std::num::NonZero;
use super::*;

mod shader;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct ComputeSetup<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a PipelineLayout>,
    shader: &'a ShaderModule,
    entry: &'a str,
    #[builder(default)]
    cache: Option<&'a PipelineCache>,
}

impl ComputeSetupBuilder<'_> {
    pub fn make(self) -> Result<ComputePipeline> {
        let built = self.build()?;
        let descriptor = ComputePipelineDescriptor {
            label: built.label,
            layout: built.layout,
            module: built.shader,
            entry_point: built.entry,
            compilation_options: Default::default(),
            cache: built.cache,
        };
        let value = built.device.create_compute_pipeline(&descriptor);
        Ok(value)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct RenderPipeSetup<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a PipelineLayout>,
    vertex: VertexState<'a>,
    fragment: Option<FragmentState<'a>>,
    #[builder(default = "PrimitiveState::default()")]
    primitive: PrimitiveState,
    #[builder(default)]
    depth_stencil: Option<DepthStencilState>,
    #[builder(default = "MultisampleState::default()")]
    multisample: MultisampleState,
    #[builder(default)]
    multiview: Option<NonZero<u32>>,
    #[builder(default)]
    cache: Option<&'a PipelineCache>,
}

impl RenderPipeSetupBuilder<'_> {
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
