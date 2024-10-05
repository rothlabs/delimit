pub use buffer::Buffer;
pub use buffer::*;
pub use bytemuck::*;
pub use flume;
pub use wgpu;
pub use wgpu::BufferUsages;

use bind::*;
use derive_builder::{Builder, UninitializedFieldError};
use encode::*;
use graph::*;
use pipe::*;
use surface::Surface;
use web_sys::HtmlCanvasElement;
use wgpu::*;

mod bind;
mod buffer;
mod encode;
mod pipe;
mod surface;

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

#[derive(Debug)]
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
        Ok(Self {
            surface: Surface::new(surface, &adapter, &device),
            device: device.into(),
            queue: queue.into(),
        })
    }
    pub fn surface(&'a self) -> &'a Surface<'a> {
        &self.surface
    }
    pub fn shader(&self, source: ShaderModuleDescriptor) -> ShaderModule {
        self.device.create_shader_module(source)
    }
    pub fn buffer(&self, size: u64) -> BufferSetupBuilder {
        BufferSetupBuilder::default()
            .device(&self.device)
            .queue(self.queue.clone())
            .size(size)
    }
    pub fn bind(&self) -> BindBuilder {
        BindBuilder::default().device(&self.device)
    }
    pub fn bind_layout(&self, entries: &'a [BindGroupLayoutEntry]) -> BindLayoutBuilder {
        BindLayoutBuilder::default().device(&self.device).entries(entries)
    }
    pub fn bind_entry(&self) -> BindEntryBuilder {
        BindEntryBuilder::default()
    }
    pub fn buffer_binding(&self) -> BufferBindingBuilder {
        BufferBindingBuilder::default()
    }
    pub fn compute_pipe(&self, shader: &'a ShaderModule) -> pipe::ComputeBuilder {
        pipe::ComputeBuilder::default()
            .device(&self.device)
            .shader(shader)
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
    pub fn vertex(&self, shader: &'a ShaderModule) -> VertexBuilder<'a> {
        VertexBuilder::default().module(shader)
    }
    pub fn fragment(&'a self, shader: &'a ShaderModule) -> FragmentBuilder<'a> {
        FragmentBuilder::default().module(shader)
    }
    pub fn attachment(&self, view: &'a TextureView) -> ColorAttachmentBuilder {
        ColorAttachmentBuilder::default().view(view)
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
