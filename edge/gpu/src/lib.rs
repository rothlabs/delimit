use std::fmt::Debug;

pub use binder::*;
pub use buffer::*;
pub use bytemuck::*;
pub use flume;
pub use surface::Surface;
pub use wgpu::{include_wgsl, BufferUsages};

use bind::*;
use derive_builder::{Builder, UninitializedFieldError};
use encode::*;
use graph::*;
use node_derive::Gate;
use pipe::*;
use shader::*;
use texture::*;
use util::DeviceExt;
use web_sys::HtmlCanvasElement;
use wgpu::*;

mod bind;
mod binder;
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

#[derive(Clone, Debug)]
pub struct Mutation;

#[derive(Clone, Debug)]
pub struct Hedge {
    pub buffer: Hub<Grc<Buffer>>,
    pub root: Hub<Mutation>,
}

impl Hedge {
    pub fn new<T>(gpu: Gpu, data: Vec<T>) -> graph::Result<Self>
    where
        T: Pod + Debug,
    {
        let size = data.len() as u64 * 4;
        let buffer: Hub<Grc<Buffer>> = gpu.buffer(size).storage_copy()?.into();
        let root = gpu.writer(buffer.clone()).data(data).hub()?;
        Ok(Self { buffer, root })
    }
}

#[derive(Clone, Debug)]
pub enum Table {
    Hedge(Hedge),
    Array(Hub<Vec<f64>>),
}

#[derive(Clone, Debug)]
pub struct Gpu {
    pub device: Grc<Device>,
    pub queue: Grc<Queue>,
    // pub adapter: Grc<Adapter>,
}

impl Gpu {
    pub async fn from_canvas<'a>(canvas: HtmlCanvasElement) -> Result<(Self, Surface<'a>)> {
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
        Ok((
            Self {
                device: grc_device.clone(),
                queue: queue.into(),
                // adapter: adapter.into(),
                //surface: Surface::new(surface, &adapter, grc_device),
            },
            Surface::new(surface, &adapter, grc_device),
        ))
    }
    pub fn shader(&self, source: ShaderModuleDescriptor) -> Shader {
        Shader {
            device: &self.device,
            inner: self.device.create_shader_module(source).into(),
        }
    }
    pub fn buffer(&self, size: u64) -> BufferRigBuilder {
        BufferRigBuilder::default()
            .device(&self.device)
            // .queue(self.queue.clone())
            .size(size)
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
        UniformBuilder::default().gpu(self.clone())
    }
    pub fn buffer_vertex<T: Pod>(&self, data: &[T]) -> Grc<Buffer> {
        self.buffer_init(data, BufferUsages::VERTEX)
    }
    pub fn bind(&self) -> BindBuilder {
        BindBuilder::default().device(&self.device)
    }
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
    pub fn encoder(&self) -> Encoder {
        Encoder {
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
            .gpu(self.clone())
            .storage(storage)
    }
    pub fn blank(&self, root: impl Into<Hub<Grc<Buffer>>>) -> BlankBuilder {
        BlankBuilder::default().gpu(self.clone()).root(root)
    }
    pub fn command(&self) -> encode::CommandBuilder {
        encode::CommandBuilder::default().gpu(self.clone())
    }
    pub fn binder(&self) -> BinderBuilder {
        BinderBuilder::default().gpu(self.clone())
    }
    pub fn hedge<T>(&self, data: Vec<T>) -> graph::Result<Hedge>
    where
        T: Pod + Debug,
    {
        Hedge::new(self.clone(), data)
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
