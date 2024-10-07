use super::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
// #[builder(setter(strip_option))]
pub struct Fragment<'a> {
    shader: &'a ShaderModule,
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
            module: built.shader,
            entry_point: built.entry,
            compilation_options: built.compilation_options,
            targets: built.targets,
        };
        Ok(state)
    }
}
