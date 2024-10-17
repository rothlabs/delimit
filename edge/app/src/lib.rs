use std::borrow::Cow;

use derive_builder::{Builder, UninitializedFieldError};
use dom::*;
use graph::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::js_sys::Math::random;
use webgl::*;
use wgpu::SurfaceTarget;
use node_derive::Gate;

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub mod demo;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error(transparent)]
    Graph(#[from] graph::Error),
    #[error("Js Error ({0})")]
    JsValue(String),
    #[error("Dom Error ({0})")]
    Dom(String),
    #[error(transparent)]
    Uninit(#[from] UninitializedFieldError),
    #[error(transparent)]
    Any(#[from] anyError),
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsValue(format!("{:?}", value))
    }
}

impl From<dom::Error> for Error {
    fn from(value: dom::Error) -> Self {
        Error::Dom(format!("{:?}", value))
    }
}

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        JsValue::from_str(&error.to_string())
    }
}

/// App Result
pub type Result<T> = std::result::Result<T, Error>;

#[wasm_bindgen(start)]
pub async fn entry() -> Result<()> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let canvas = Window::new()?
        .document()?
        .body()?
        .stem("canvas")?
        .canvas()?;

    let instance = wgpu::Instance::default();
    let surface_target = SurfaceTarget::Canvas(canvas.object);
    let surface = instance.create_surface(surface_target).unwrap();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            // Request an adapter which can render to our surface
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
                required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                memory_hints: wgpu::MemoryHints::MemoryUsage,
            },
            None,
        )
        .await
        .expect("Failed to create device");
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(SHADER)),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            compilation_options: Default::default(),
            targets: &[Some(swapchain_format.into())],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = surface.get_default_config(&adapter, 300, 150).unwrap();
    surface.configure(&device, &config);

    ///////////////////// Render
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_pipeline(&render_pipeline);
        rpass.draw(0..3, 0..1);
    }
    queue.submit(Some(encoder.finish()));
    frame.present();

    console_log!("Adapter: {:?}", adapter.get_info());

    demo::nurbs::DemoBuilder::default()
        .curves(40)
        .duration(5000.)
        .width(1200)
        .height(900)
        .make()?
        .start();
    Ok(())
}

pub fn random_float() -> f32 {
    random() as f32 * 2. - 1.
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[macro_use]
extern crate macro_rules_attribute;

pub const SHADER: &str = r"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
";
