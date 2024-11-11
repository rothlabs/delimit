use std::ops::Range;
use super::*;

// #[derive(Builder, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct Command {
//     core: Core,
//     #[builder(default, setter(each(name = "root", into)))]
//     roots: Vec<Hub<Mutation>>,
//     // TODO: take enum of either DIRECT(Grc<TextureView>) or RESOLVE(Grc<TextureView>, Grc<TextureView>)
//     #[builder(default)]
//     display: Option<Grc<Display>>,
//     #[builder(default)]
//     texture_view: Option<Grc<TextureView>>,
//     #[builder(default)]
//     resolve_target: Option<Grc<TextureView>>,
//     #[builder(default, setter(each(name = "compute_command", into)))]
//     compute_commands: Vec<ComputeCommand>,
//     #[builder(default, setter(each(name = "render_command", into)))]
//     render_commands: Vec<RenderCommand>,
// }


#[derive(Debug)]
enum ComputeCommand {
    Pipe(Grc<ComputePipeline>),
    Bind(u32, Grc<BindGroup>),
    Dispatch(u32),
}

#[derive(Debug)]
enum RenderCommand {
    Pipe(Grc<RenderPipeline>),
    Bind(u32, Grc<BindGroup>),
    Vertex(u32, Grc<Buffer>),
    Index(Grc<Buffer>),
    Draw(Range<u32>, Range<u32>),
    DrawIndexed((Range<u32>, i32, Range<u32>)),
}