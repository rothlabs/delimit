use super::*;

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Dispatch {
    #[builder(setter(each(name = "stem", into)), default)]
    stems: Vec<Hub<Grc<Action>>>,
    pipe: Hub<Grc<ComputePipeline>>,
    #[builder(setter(each(name = "bind", into)))]
    bindings: Vec<Hub<action::Bind>>,
    size: Hub<u32>,
}

impl Solve for Dispatch {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let dispatch = pack::Dispatch {
            stems: self.stems.base().await?,
            pipe: self.pipe.base().await?,
            binds: self.bindings.base().await?,
            size: self.size.base().await?,
        };
        Ok(Grc::new(Action::Dispatch(dispatch)).into())
    }
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Bind {
    #[builder(default)]
    slot: Hub<u32>,
    group: Hub<Grc<BindGroup>>,
    #[builder(default)]
    offsets: Hub<Vec<u32>>,
}

impl Solve for Bind {
    type Base = action::Bind;
    async fn solve(&self) -> node::Result<action::Bind> {
        let binding = action::Bind {
            slot: self.slot.base().await?,
            group: self.group.base().await?,
            offsets: self.offsets.base().await?,
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
    vertex: Hub<action::Vertex>,
    vertices: Hub<Range<u32>>,
    instances: Hub<Range<u32>>,
}

impl Solve for Draw {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let draw = pack::Draw {
            stems: self.stems.base().await?,
            pipe: self.pipe.base().await?,
            binds: self.binds.base().await?,
            vertex: self.vertex.base().await?,
            vertices: self.vertices.base().await?,
            instances: self.instances.base().await?,
        };
        Ok(Grc::new(Action::Draw(draw)).into())
    }
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(setter(into), pattern = "owned")]
pub struct Vertex {
    #[builder(default)]
    slot: Hub<u32>,
    buffer: Hub<Grc<Buffer>>,
}

impl Solve for Vertex {
    type Base = action::Vertex;
    async fn solve(&self) -> node::Result<action::Vertex> {
        let vertex = action::Vertex {
            slot: self.slot.base().await?,
            buffer: self.buffer.base().await?,
        };
        Ok(vertex.into())
    }
}

// #[builder(default, setter(each(name = "bind_inner", into)))]
//     bindings: Vec<Hub<action::Binding>>,

// impl DispatchBuilder {
//     pub fn bind(self, slot: impl Into<Hub<u32>>, group: impl Into<Hub<Grc<BindGroup>>>, offsets: impl Into<Hub<Vec<u32>>>) -> Self {
//         self.bind_inner(BindingBuilder::default().slot(slot).group(group).offsets(offsets).hub().unwrap())
//     }
// }

// self.bind_inner(Binding {slot: slot.into(), group: group.into(), offsets: offsets.into()}.hub().unwrap())

// bind: Vec<(Hub<u32>, Hub<Grc<BindGroup>>, Hub<Vec<u32>>)>,
