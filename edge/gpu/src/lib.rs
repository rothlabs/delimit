pub use buffer::Buffer;
pub use buffer::*;
pub use bytemuck::*;
pub use flume;
use util::DeviceExt;
pub use wgpu::BufferUsages;

use bind::*;
use derive_builder::{Builder, UninitializedFieldError};
use encode::*;
use graph::*;
use pipe::*;
use shader::*;
use surface::Surface;
use texture::*;
use web_sys::HtmlCanvasElement;
use wgpu::*;

mod bind;
mod buffer;
mod encode;
mod pipe;
mod shader;
mod surface;
mod texture;

#[macro_use]
extern crate macro_rules_attribute;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyError),
}

pub type Result<T> = std::result::Result<T, Error>;

// #[derive(Debug)]
pub struct Gpu<'a> {
    pub device: Grc<Device>,
    pub queue: Grc<Queue>,
    surface: Surface<'a>,
}

impl<'a> Gpu<'a> {
    pub async fn from_canvas(canvas: HtmlCanvasElement) -> Result<Self> {
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
            surface: Surface::new(surface, &adapter, grc_device),
        })
    }
    pub fn surface(&'a self) -> &'a Surface<'a> {
        &self.surface
    }
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.device,
            inner: self.device.create_shader_module(source).into(),
        }
    }
    pub fn buffer(&self, size: u64) -> BufferSetupBuilder {
        BufferSetupBuilder::default()
            .device(&self.device)
            .queue(self.queue.clone())
            .size(size)
    }
    fn buffer_init<T: NoUninit>(&self, data: &[T], usage: BufferUsages) -> crate::Buffer {
        let inner = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Uniform Buffer"),
                contents: bytemuck::cast_slice(data),
                usage, //: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            });
        Buffer {
            inner: inner.into(),
            queue: self.queue.clone(),
        }
    }
    pub fn buffer_uniform<T: NoUninit>(&self, data: &[T]) -> crate::Buffer {
        self.buffer_init(data, BufferUsages::UNIFORM | BufferUsages::COPY_DST)
    }
    pub fn buffer_vertex<T: NoUninit>(&self, data: &[T]) -> crate::Buffer {
        self.buffer_init(data, BufferUsages::VERTEX)
    }
    pub fn bind(&self) -> BindBuilder {
        BindBuilder::default().device(&self.device)
    }
    pub fn bind_layout(&self, entries: &'a [BindGroupLayoutEntry]) -> BindLayoutBuilder {
        BindLayoutBuilder::default()
            .device(&self.device)
            .entries(entries)
    }
    pub fn bind_entry(&self, binding: u32, ty: BindingType) -> BindEntryBuilder {
        BindEntryBuilder::default().binding(binding).ty(ty)
    }
    pub fn uniform(&self) -> BufferBindingBuilder {
        BufferBindingBuilder::default().ty(BufferBindingType::Uniform)
    }
    pub fn storage(&self, read_only: bool) -> BufferBindingBuilder {
        BufferBindingBuilder::default().ty(BufferBindingType::Storage { read_only })
    }
    pub fn pipe_layout(&self, bind_layout: &'a [&'a BindGroupLayout]) -> pipe::LayoutBuilder {
        pipe::LayoutBuilder::default()
            .device(&self.device)
            .bind_layouts(bind_layout)
    }
    pub fn render_pipe(&self, vertex: VertexState<'a>) -> pipe::RenderBuilder {
        pipe::RenderBuilder::default()
            .device(&self.device)
            .vertex(vertex)
    }
    pub fn render_pass(
        &self,
        attachments: &'a [Option<RenderPassColorAttachment<'a>>],
    ) -> encode::RenderBuilder {
        encode::RenderBuilder::default().attachments(attachments)
    }
    pub fn encoder(&self) -> Encoder {
        Encoder {
            inner: self
                .device
                .create_command_encoder(&CommandEncoderDescriptor::default()),
            queue: &self.queue,
        }
    }
    pub fn attachment(&self, view: &'a TextureView) -> ColorAttachmentBuilder {
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
}

// #[derive(ThisError, Debug)]
// pub enum Error {
//     #[error(transparent)]
//     Graph(#[from] graph::Error),
//     #[error(transparent)]
//     CreateSurfaceError(#[from] CreateSurfaceError),
//     #[error(transparent)]
//     Uninit(#[from] UninitializedFieldError),
//     #[error(transparent)]
//     BufferAsync(#[from] BufferAsyncError),
//     #[error(transparent)]
//     Any(#[from] anyError),
// }
