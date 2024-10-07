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
#[builder(setter(strip_option))]
pub struct Layout<'a> {
    array_stride: u64,
    #[builder(default)]
    step_mode: VertexStepMode,
    attributes: &'a [VertexAttribute],
}

impl<'a> LayoutBuilder<'a> {
    pub fn make(self) -> Result<VertexBufferLayout<'a>> {
        let built = self.build()?;
        let out = VertexBufferLayout {
            array_stride: built.array_stride,
            step_mode: built.step_mode,
            attributes: built.attributes,
        };
        Ok(out)
    }
    pub fn list(self) -> Result<[VertexBufferLayout<'a>; 1]> {
        Ok([self.make()?])
    }
}
