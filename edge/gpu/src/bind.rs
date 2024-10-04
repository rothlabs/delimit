use super::*;

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
#[builder(setter(strip_option))]
pub struct BindGroup<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a wgpu::BindGroupLayout>,
    #[builder(default)]
    entries: Vec<BindGroupEntry<'a>>,
    #[builder(default)]
    pipeline: Option<&'a ComputePipeline>,
}

impl BindGroup<'_> {
    fn make(self, layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let descriptor = BindGroupDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        };
        self.device.create_bind_group(&descriptor)
    }
}

impl<'a> BindGroupBuilder<'a> {
    pub fn make(self) -> Result<wgpu::BindGroup> {
        let built = self.build()?;
        if let Some(layout) = built.layout {
            Ok(built.make(layout))
        } else if let Some(pipe) = built.pipeline {
            Ok(built.make(&pipe.get_bind_group_layout(0)))
        } else {
            Err(anyhow!("no layout for bind group"))?
        }
    }
    pub fn entry(mut self, binding: u32, buffer: &'a crate::Buffer) -> Self {
        let resource = buffer.resource();
        if let Some(mut entries) = self.entries {
            entries.push(BindGroupEntry { binding, resource });
            self.entries = Some(entries);
        } else {
            self.entries = Some(vec![BindGroupEntry { binding, resource }]);
        }
        self
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "crate::Error"))]
pub struct BindGroupLayout<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default, setter(each(name = "entry")))]
    entries: &'a [BindGroupLayoutEntry],
}

impl BindGroupLayoutBuilder<'_> {
    pub fn make(self) -> Result<wgpu::BindGroupLayout> {
        let built = self.build()?;
        let descriptor = BindGroupLayoutDescriptor {
            label: built.label,
            entries: built.entries,
        };
        let value = built.device.create_bind_group_layout(&descriptor);
        Ok(value)
    }
}

// pub fn make(self) -> Result<wgpu::BindGroup> {
//     let built = self.build()?;
//     let mut layout = built.layout;
//     if layout.is_none() {
//         if let Some(pipe) = built.pipe {
//             let crap = pipe.get_bind_group_layout(0);

//             layout = Some(&crap);
//         }
//     }
//     if let Some(layout) = layout {
//         let descriptor = BindGroupDescriptor {
//             label: built.label,
//             layout, //: built.layout,
//             entries: &built.entries,
//         };
//         let value = built.device.create_bind_group(&descriptor);
//         Ok(value)
//     } else {
//         Err(anyhow!("no layout for bind group"))?
//     }
// }
