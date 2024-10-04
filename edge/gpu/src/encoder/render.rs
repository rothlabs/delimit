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
    pub fn dispatch(mut self, vertices: Range<u32>, instances: Range<u32>) -> Self {
        self.0.draw(vertices, instances);
        self
    }
}

// #[derive(Builder, Debug)]
// #[builder(pattern = "owned")]
// #[builder(build_fn(error = "crate::Error"))]
// pub struct RenderSetup<'a> {
//     // module: &'a ShaderModule,
//     // entry: &'a str,
//     // #[builder(default)]
//     // compilation_options: PipelineCompilationOptions<'a>,
//     // #[builder(default)]
//     // buffers: &'a [VertexBufferLayout<'a>],
// }

// impl<'a> RenderSetupBuilder<'a> {
//     pub fn make(self) -> Result<RenderPassDescriptor<'a>> {
//         let built = self.build()?;
//         let state = RenderPassDescriptor {
            
//         };
//         Ok(state)
//     }
// }