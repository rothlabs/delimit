use super::*;

pub trait ToCore {
    fn gpu(self) -> impl Future<Output = Result<Core>>;
}

impl ToCore for Adapter {
    async fn gpu(self) -> Result<Core> {
        let descriptor = DeviceDescriptor {
            required_limits: Limits::default().using_resolution(self.limits()),
            ..Default::default()
        };
        let (device, queue) = self.request_device(&descriptor, None).await?;
        let store = Store::new(&device);
        Ok(Core {
            adapter: self.into(),
            device: device.into(),
            queue: queue.into(),
            store,
        })
    }
}

// TODO: Core should directly hold adapter, device, queue and be used as Grc<Gpu>
#[derive(Clone, Debug)]
pub struct Core {
    pub adapter: Grc<Adapter>,
    pub device: Grc<Device>,
    pub queue: Grc<Queue>,
    pub store: Store,
}

impl Core {
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.device,
            module: self.device.create_shader_module(source), //.into(),
            targets: &[],
        }
    }
    pub fn hedge<T>(&self, data: Vec<T>) -> Result<Hedge>
    where
        T: Pod + Debug + graph::SendSync,
    {
        let size: Hub<u32> = (data.len() as u32).into();
        let offset = self.store.storage(&size);
        let buffer = self.store.storage.buffer.hub();
        // TODO: make self.uniform_writer
        let stem = self.writer(buffer).offset(&offset).data(data).hub()?;
        Ok(Hedge {
            offset,
            size,
            stems: vec![stem],
        })
    }
    pub fn buffer(&self, size: u64) -> BufferRigBuilder {
        BufferRigBuilder::default().device(&self.device).size(size)
    }
    fn buffer_init<T: Pod>(&self, data: &[T], usage: BufferUsages) -> Grc<Buffer> {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(data),
                usage,
            })
            .into()
    }
    pub fn uniform<T: Pod>(&self) -> UniformBuilder<T> {
        UniformBuilder::default().core(self.clone())
    }
    pub fn vertex_buffer<T: Pod>(&self, data: &[T]) -> Grc<Buffer> {
        self.buffer_init(data, BufferUsages::VERTEX)
    }
    pub fn index_buffer<T: Pod>(&self, data: &[T]) -> Grc<Buffer> {
        self.buffer_init(data, BufferUsages::INDEX)
    }
    // TODO: bind::LayoutBuilder
    pub fn bind_layout<'a>(&'a self, entries: &'a [BindGroupLayoutEntry]) -> BindLayoutBuilder {
        BindLayoutBuilder::default()
            .device(&self.device)
            .entries(entries)
    }
    pub fn bind_entry(&self, binding: u32, ty: BindingType) -> BindEntryBuilder {
        BindEntryBuilder::default().binding(binding).ty(ty)
    }
    pub fn bind_uniform(&self) -> BufferBindingBuilder {
        BufferBindingBuilder::default().ty(BufferBindingType::Uniform)
    }
    pub fn bind_storage(&self, read_only: bool) -> BufferBindingBuilder {
        BufferBindingBuilder::default().ty(BufferBindingType::Storage { read_only })
    }
    pub fn pipe_layout<'a>(
        &'a self,
        bind_layout: &'a [&'a BindGroupLayout],
    ) -> pipe::LayoutBuilder {
        pipe::LayoutBuilder::default()
            .device(&self.device)
            .bind_layouts(bind_layout)
    }
    // TODO: put this in viewport?
    pub fn render_pass<'a>(
        &'a self,
        attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    ) -> encode::RenderBuilder {
        encode::RenderBuilder::default().attachments(attachments)
    }
    pub fn encoder(&self) -> Encode {
        Encode {
            inner: self
                .device
                .create_command_encoder(&CommandEncoderDescriptor::default()),
            queue: &self.queue,
        }
    }
    pub fn attachment<'a>(&'a self, view: &'a TextureView) -> ColorAttachmentBuilder {
        ColorAttachmentBuilder::default().view(view)
    }
    pub fn lines(&self) -> PrimitiveBuilder {
        PrimitiveBuilder::default().topology(PrimitiveTopology::LineList)
    }
    pub fn vertex_layout(&self, array_stride: u64) -> pipe::vertex::LayoutBuilder {
        pipe::vertex::LayoutBuilder::default().array_stride(array_stride)
    }
    pub fn multisample(&self, count: u32) -> MultisampleBuilder {
        MultisampleBuilder::default().count(count)
    }
    pub fn writer<T>(&self, buffer: impl Into<Hub<Grc<Buffer>>>) -> BufferWriterBuilder<T> {
        BufferWriterBuilder::default()
            .queue(self.queue.clone())
            .buffer(buffer)
    }
    pub fn reader<T>(&self, storage: impl Into<Hub<Grc<Buffer>>>) -> BufferReaderBuilder<T> {
        BufferReaderBuilder::default()
            .core(self.clone())
            .storage(storage)
    }
    pub fn blank(&self, size: impl Into<Hub<u32>>) -> BlankBuilder {
        BlankBuilder::default().core(self.clone()).size(size)
    }
    pub fn size(&self, buffer: impl Into<Hub<Grc<Buffer>>>) -> SizeBuilder {
        SizeBuilder::default().buffer(buffer)
    }
    // pub fn compute(&self) -> encode::ComputeBuilder {
    //     encode::ComputeBuilder::default().core(self.clone())
    // }
    // pub fn bind2(&self) -> bind::BindGroupUnitBuilder<((Grc<Device>,), (), ())> {
    //     bind::BindGroupUnit::builder().device(self.device.clone())
    // }
    pub fn bind(&self) -> bind::BindGroupRigBuilder {
        bind::BindGroupRigBuilder::default().device(self.device.clone())
    }
    /// Create a dummy `Hedge` to quickly put data into GPU ecosystem.
    pub fn buffer_hedge<T>(&self, data: Vec<T>) -> Result<BufferHedge>
    where
        T: Pod + Debug + graph::SendSync,
    {
        let size = data.len() as u64 * 4;
        let buffer: Hub<Grc<Buffer>> = self.buffer(size).storage()?.into();
        let stem = self.writer(buffer.clone()).data(data).hub()?;
        bufferhedge().buffer(buffer).stem(stem).build()
        // Ok(Hedge { buffer, stem })
    }
}

// pub fn uniform_hedge<T>(&self, data: Vec<T>) -> Result<Hedge>
//     where
//         T: Pod + Debug + graph::SendSync,
//     {
//         let size: Hub<u32> = (data.len() as u32).into();
//         let offset = self.store.uniform(&size);
//         let buffer = self.store.uniform.buffer.hub();
//         // TODO: make self.uniform_writer
//         let stem = self.writer(buffer).offset(&offset).data(data).hub()?;
//         Ok(Hedge {
//             offset,
//             size,
//             stems: vec![stem],
//         })
//     }

// pub fn render_pipe<'a>(&'a self, vertex: VertexState<'a>) -> pipe::RenderBuilder {
//     pipe::RenderBuilder::default()
//         .device(&self.device)
//         .vertex(vertex)
// }

// pub async fn from_surface(instance: Instance, surface: Grc<Surface<'static>>) -> Result<Self> {
//     let mut descriptor = RequestAdapterOptions::default();
//     descriptor.compatible_surface = Some(&surface);
//     let adapter = instance
//         .request_adapter(&descriptor)
//         .await
//         .ok_or(anyhow!("no adapter"))?;
//     let mut descriptor = DeviceDescriptor::default();
//     descriptor.required_limits = Limits::default().using_resolution(adapter.limits());
//     let (device, queue) = adapter.request_device(&descriptor, None).await?;
//     Ok(Self {
//         // display: Display::new(surface, &adapter).into(),
//         device: device.into(),
//         adapter: adapter.into(),
//         queue: queue.into(),
//         // chain: Leaf::default(),
//     })
// }

// #[cfg(target_arch = "wasm32")]
// use web_sys::HtmlCanvasElement;

// #[cfg(target_arch = "wasm32")]
//     pub async fn from_canvas<'a>(canvas: HtmlCanvasElement) -> Result<Self> {
//         let instance = Instance::default();
//         let surface_target = SurfaceTarget::Canvas(canvas);
//         let surface = instance.create_surface(surface_target)?;
//         let adapter = instance
//             .request_adapter(&RequestAdapterOptions {
//                 power_preference: PowerPreference::default(),
//                 force_fallback_adapter: false,
//                 compatible_surface: Some(&surface),
//             })
//             .await
//             .expect("Failed to find an appropriate adapter");
//         let required_limits = Limits::default().using_resolution(adapter.limits());
//         let (device, queue) = adapter
//             .request_device(
//                 &DeviceDescriptor {
//                     label: None,
//                     required_features: Features::empty(),
//                     required_limits,
//                     memory_hints: MemoryHints::MemoryUsage,
//                 },
//                 None,
//             )
//             .await
//             .expect("Failed to create device");
//         let grc_device = Grc::new(device);
//         Ok(Self {
//             device: grc_device.clone(),
//             queue: post.into(),
//             display: Display::new(surface, &adapter, grc_device).into(),
//         })
//     }
