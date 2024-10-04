use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Compute<'a> {
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

impl ComputeBuilder<'_> {
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
