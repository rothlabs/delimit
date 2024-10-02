pub use buffer::Buffer;
pub use buffer::*;
pub use wgpu;
pub use wgpu::BufferUsages;
pub use flume;
pub use bytemuck;

use bind::*;
use derive_builder::{Builder, UninitializedFieldError};
use graph::*;
use pipe::*;
use web_sys::HtmlCanvasElement;
use wgpu::*;
use encoder::*;

mod bind;
mod buffer;
mod pipe;
mod encoder;

#[macro_use]
extern crate macro_rules_attribute;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error(transparent)]
    CreateSurfaceError(#[from] CreateSurfaceError),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Gpu {
    pub device: Grc<Device>,
    pub queue: Grc<Queue>,
}

impl Gpu {
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
            device: device.into(),
            queue: queue.into(),
        })
    }
    pub fn shader(&self, source: ShaderModuleDescriptor) -> ShaderModule {
        self.device.create_shader_module(source)
    }
    pub fn buffer(&self) -> BufferRubricBuilder {
        BufferRubricBuilder::default()
            .device(&self.device)
            .queue(self.queue.clone())
    }
    pub fn bind_group(&self) -> BindGroupBuilder {
        BindGroupBuilder::default().device(&self.device)
    }
    pub fn bind_group_layout(&self) -> BindGroupLayoutBuilder {
        BindGroupLayoutBuilder::default().device(&self.device)
    }
    pub fn compute(&self) -> ComputeBuilder {
        ComputeBuilder::default().device(&self.device)
    }
    pub fn encoder(&self) -> Encoder {
        Encoder {
            inner: self.device
                    .create_command_encoder(&CommandEncoderDescriptor { label: None }),
            queue: &self.queue
        }
    }
    // pub fn encoder(&self) -> CommandEncoder {
    //     self.device
    //         .create_command_encoder(&CommandEncoderDescriptor { label: None })
    // }
    // pub fn submit(&self, encoder: CommandEncoder) -> SubmissionIndex {
    //     self.queue.submit(Some(encoder.finish()))
    // }
}
