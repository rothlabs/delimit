#[derive(Builder, Debug)]
#[builder(pattern = "owned")]
#[builder(build_fn(error = "graph::Error"))]
#[builder(setter(strip_option))]
pub struct Bind<'a> {
    device: &'a Device,
    #[builder(default)]
    label: Option<&'a str>,
    #[builder(default)]
    layout: Option<&'a BindGroupLayout>,
    #[builder(default)]
    entries: Vec<BindGroupEntry<'a>>,
    #[builder(default)]
    pipe: Option<&'a ComputePipeline>,
}

impl Bind<'_> {
    fn make(self, layout: &BindGroupLayout) -> Grc<BindGroup> {
        let descriptor = BindGroupDescriptor {
            label: self.label,
            layout,
            entries: &self.entries,
        };
        self.device.create_bind_group(&descriptor).into()
    }
}

impl<'a> BindBuilder<'a> {
    pub fn make(self) -> graph::Result<Grc<BindGroup>> {
        let built = self.build()?;
        if let Some(layout) = built.layout {
            Ok(built.make(layout))
        } else if let Some(pipe) = built.pipe {
            Ok(built.make(&pipe.get_bind_group_layout(0)))
        } else {
            Err(anyhow!("no layout for bind group"))?
        }
    }
    pub fn entry(mut self, binding: u32, buffer: &'a Buffer) -> Self {
        let resource = buffer.as_entire_binding();
        if let Some(mut entries) = self.entries {
            entries.push(BindGroupEntry { binding, resource });
            self.entries = Some(entries);
        } else {
            self.entries = Some(vec![BindGroupEntry { binding, resource }]);
        }
        self
    }
}