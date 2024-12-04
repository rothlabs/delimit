use typed_builder::TypedBuilder;

use super::*;

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Dispatch {
    #[builder(setter(each(name = "stem", into)), default)]
    stems: Vec<Hub<Grc<Action>>>,
    pipe: Hub<Grc<ComputePipeline>>,
    #[builder(setter(each(name = "bind", into)))]
    binds: Vec<Hub<action::Bind>>,
    size: Hub<u32>,
}

impl Solve for Dispatch {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let size = self.size.base().await?;
        let compute = pack::pass::Compute {
            pipe: self.pipe.base().await?,
            binds: self.binds.base().await?,
            kind: pack::pass::compute::Kind::Dispatch(size),
        };
        let action = pack::Action {
            stems: self.stems.base().await?,
            kind: pack::Kind::Compute(compute),
            ..Default::default()
        };
        Ok(Grc::new(action).into())
    }
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Bind {
    #[builder(default)]
    slot: Hub<u32>,
    group: Hub<Grc<BindGroup>>,
    #[builder(default, setter(each(name = "offset", into)))]
    offsets: Vec<Hub<u32>>,
}

impl Solve for Bind {
    type Base = action::Bind;
    async fn solve(&self) -> node::Result<action::Bind> {
        let mut offsets = self.offsets.base().await?;
        for offset in &mut offsets {
            *offset *= 4;
        }
        let binding = action::Bind {
            slot: self.slot.base().await?,
            group: self.group.base().await?,
            offsets,
        };
        Ok(binding.into())
    }
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Draw {
    stems: Vec<Hub<Grc<Action>>>,
    pipe: Hub<Grc<RenderPipeline>>,
    #[builder(setter(each(name = "bind", into)))]
    binds: Vec<Hub<action::Bind>>,
    #[builder(setter(each(name = "buffer", into)))]
    buffers: Vec<Hub<action::Vertex>>,
    vertex_offset: Hub<u32>,
    vertex_length: Hub<u32>,
    instance_offset: Hub<u32>,
    instance_length: Hub<u32>,
}

impl Solve for Draw {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let draw = action::Draw {
            vertices: self.vertex_offset.base().await?..self.vertex_length.base().await?,
            instances: self.instance_offset.base().await?..self.instance_length.base().await?,
        };
        let render = pack::pass::Render {
            pipe: self.pipe.base().await?,
            binds: self.binds.base().await?,
            buffers: self.buffers.base().await?,
            kind: pack::pass::render::Kind::Draw(draw),
        };
        let action = pack::Action {
            stems: self.stems.base().await?,
            kind: pack::Kind::Render(render),
            ..Default::default()
        };
        Ok(Grc::new(action).into())
    }
}

#[derive(Debug, Gate, Back, TypedBuilder)]
pub struct Vertex {
    #[builder(default)]
    pub slot: Hub<u32>,
    #[builder(setter(into))]
    pub buffer: Hub<Grc<Buffer>>,
    #[builder(default)]
    pub offset: Option<Hub<u32>>,
}

impl Solve for Vertex {
    type Base = action::Vertex;
    async fn solve(&self) -> node::Result<action::Vertex> {
        let vertex = action::Vertex {
            slot: self.slot.base().await?,
            buffer: self.buffer.base().await?,
            offset: self.offset.base().await?,
        };
        Ok(vertex.into())
    }
}

// // TODO: this could be provided in Store?
// #[derive(Debug, Back, Builder, BuildGate, Make)]
// #[builder(setter(strip_option, into), pattern = "owned")]
// pub struct Vertex {
//     #[builder(default)]
//     slot: Hub<u32>,
//     buffer: Hub<Grc<Buffer>>,
//     #[builder(default)]
//     offset: Option<Hub<u32>>,
// }
