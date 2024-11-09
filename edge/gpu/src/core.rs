use super::*;
use display::Display;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
use winit::window::Window;

#[derive(Clone, Debug)]
pub struct Core {
    pub device: Grc<Device>,
    pub queue: Grc<Queue>,
    pub display: Grc<Display>,
}

impl Core {
    pub async fn from_window(window: Grc<Window>) -> Result<Self> {
        let instance = Instance::default();
        let surface_target = SurfaceTarget::Window(Box::new(window));
        let surface = instance.create_surface(surface_target)?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        let required_limits = Limits::default().using_resolution(adapter.limits());
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits,
                    memory_hints: MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .expect("Failed to create device");
        let grc_device = Grc::new(device);
        Ok(Self {
            device: grc_device.clone(),
            queue: queue.into(),
            display: Display::new(surface, &adapter, grc_device).into(),
        })
    }
    #[cfg(target_arch = "wasm32")]
    pub async fn from_canvas<'a>(canvas: HtmlCanvasElement) -> Result<Self> {
        let instance = Instance::default();
        let surface_target = SurfaceTarget::Canvas(canvas);
        let surface = instance.create_surface(surface_target)?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        let required_limits = Limits::default().using_resolution(adapter.limits());
        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits,
                    memory_hints: MemoryHints::MemoryUsage,
                },
                None,
            )
            .await
            .expect("Failed to create device");
        let grc_device = Grc::new(device);
        Ok(Self {
            device: grc_device.clone(),
            queue: queue.into(),
            display: Display::new(surface, &adapter, grc_device).into(),
        })
    }
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.device,
            inner: self.device.create_shader_module(source).into(),
        }
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
    pub fn render_pipe<'a>(&'a self, vertex: VertexState<'a>) -> pipe::RenderBuilder {
        pipe::RenderBuilder::default()
            .device(&self.device)
            .vertex(vertex)
    }
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
    pub fn command(&self) -> encode::CommandBuilder {
        encode::CommandBuilder::default().core(self.clone())
    }
    pub fn bind(&self) -> BindBuilder {
        BindBuilder::default().device(self.device.clone())
    }
    /// Create a dummy `Hedge` to quickly put data into GPU ecosystem.
    pub fn hedge<T>(&self, data: Vec<T>) -> Result<Hedge>
    where
        T: Pod + Debug + graph::SendSync,
    {
        let size = data.len() as u64 * 4;
        let buffer: Hub<Grc<Buffer>> = self.buffer(size).storage()?.into();
        let root = self.writer(buffer.clone()).data(data).hub()?;
        Ok(Hedge { buffer, root })
    }
}
