use gpu::Gpu;
use graph::*;
use wgpu::*;
// use win::Win;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[tokio::main] // (flavor = "current_thread")
async fn main() {
    let mut app = win::App::default();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app).unwrap();
}

// fn start(instance: Instance, surface: Surface<'static>) {
//     tokio::task::spawn(async move {
//         let gpu = Gpu::from_surface(instance, surface).await.unwrap();
//         draw_triangle(gpu).await.unwrap();
//     });
// }

// async fn draw_triangle(gpu: Gpu) -> gpu::Result<()> {
//     let targets = gpu.display.targets();
//     let shader = gpu.shader(include_wgsl!("triangle.wgsl"));
//     let vertex = shader.vertex("vs_main").make()?;
//     let fragment = shader.fragment("fs_main").targets(targets).make()?;
//     let pipe = gpu.render_pipe(vertex).fragment(fragment).make()?;
//     // let view = gpu.display.view();
//     gpu.command()
//         .display(gpu.display.clone())
//         // .texture_view(view)
//         .render(pipe)
//         .draw(0..3, 0..1)
//         .hub()?
//         .base()
//         .await?;
//     println!("draw triangle complete");
//     Ok(())
// }
