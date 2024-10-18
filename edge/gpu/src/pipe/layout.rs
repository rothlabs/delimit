use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct Layout<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    bind_layouts: &'a [&'a BindGroupLayout],// Vec<Grc<BindGroupLayout>>,
    #[builder(default)]
    push_constant_ranges: &'a [PushConstantRange],
}

impl LayoutBuilder<'_> {
    pub fn make(self) -> Result<PipelineLayout> {
        let built = self.build()?;
        //let layouts: Vec<&BindGroupLayout> = built.bind_layouts.iter().map(|x| x.as_ref()).collect();
        let descriptor = PipelineLayoutDescriptor {
            label: built.label,
            bind_group_layouts: built.bind_layouts,//&layouts,
            push_constant_ranges: built.push_constant_ranges,
        };
        let value = built.device.create_pipeline_layout(&descriptor);
        Ok(value)
    }
}


// #[derive(Builder, Debug)]
// #[builder(pattern = "owned")]
// #[builder(build_fn(error = "crate::Error"))]
// #[builder(setter(strip_option))]
// pub struct Layout<'a> {
//     device: &'a Device,
//     #[builder(default)]
//     label: Option<&'a str>,
//     bind_layouts: &'a [&'a BindGroupLayout],
//     #[builder(default)]
//     push_constant_ranges: &'a [PushConstantRange],
// }

// impl LayoutBuilder<'_> {
//     pub fn make(self) -> Result<PipelineLayout> {
//         let built = self.build()?;
//         let descriptor = PipelineLayoutDescriptor {
//             label: built.label,
//             bind_group_layouts: built.bind_layouts,
//             push_constant_ranges: built.push_constant_ranges,
//         };
//         let value = built.device.create_pipeline_layout(&descriptor);
//         Ok(value)
//     }
// }