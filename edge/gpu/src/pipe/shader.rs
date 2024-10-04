use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Vertex<'a> {
    module: &'a ShaderModule,
    entry: &'a str,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(default)]
    buffers: &'a [VertexBufferLayout<'a>],
}

impl<'a> VertexBuilder<'a> {
    pub fn make(self) -> Result<VertexState<'a>> {
        let built = self.build()?;
        let state = VertexState {
            module: built.module,
            entry_point: built.entry,
            compilation_options: built.compilation_options,
            buffers: built.buffers,
        };
        Ok(state)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Fragment<'a> {
    module: &'a ShaderModule,
    entry: &'a str,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(default)]
    targets: &'a [Option<ColorTargetState>],
}

impl<'a> FragmentBuilder<'a> {
    pub fn make(self) -> Result<FragmentState<'a>> {
        let built = self.build()?;
        let state = FragmentState {
            module: built.module,
            entry_point: built.entry,
            compilation_options: built.compilation_options,
            targets: built.targets,
        };
        Ok(state)
    }
}
