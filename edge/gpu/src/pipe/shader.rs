use super::*;

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct Vertex<'a> {
    shader: &'a ShaderModule,
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
            module: built.shader,
            entry_point: built.entry,
            compilation_options: built.compilation_options,
            buffers: built.buffers,
        };
        Ok(state)
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
// #[builder(setter(strip_option))]
pub struct Fragment<'a> {
    shader: &'a Shader<'a>,
    entry: &'a str,
    #[builder(default)]
    compilation_options: PipelineCompilationOptions<'a>,
    #[builder(default)]
    targets: &'a [Option<ColorTargetState>],
    local_surface: &'a crate::Surface<'a>,
}

impl<'a> FragmentBuilder<'a> {
    pub fn make(self) -> Result<FragmentState<'a>> {
        let built = self.build()?;
        let state = FragmentState {
            module: &built.shader,
            entry_point: built.entry,
            compilation_options: built.compilation_options,
            targets: built.targets,
        };
        Ok(state)
    }
    pub fn surface(self) -> Result<FragmentState<'a>> {
        if let Some(surface) = self.local_surface {
            self.targets(surface.targets()).make()
        } else {
            self.make()
        }
    }
}
