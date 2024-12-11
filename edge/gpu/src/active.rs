use super::*;

pub mod group;

#[derive(Debug, Gate)]
pub struct Group {
    pub device: Grc<Device>,
    pub layout: Grc<BindGroupLayout>,
    pub entries: Vec<group::Entry>,
}

impl Solve for Group {
    type Base = Grc<wgpu::BindGroup>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let mut buffers = vec![];
        for entry in &self.entries {
            buffers.push(entry.buffer.base().await?);
        }
        let mut entries = vec![];
        for (entry, buffer) in self.entries.iter().zip(&buffers) {
            entries.push(BindGroupEntry {
                binding: entry.slot,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer,
                    offset: 0,
                    size: entry.size,
                }),
            });
        }
        let bind = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.layout,
            entries: &entries,
        });
        Ok(Grc::new(bind).into())
    }
}

impl Adapt for Group {
    fn back(&mut self, back: &Back) {
        for entry in &mut self.entries {
            entry.buffer.back(back);
        }
    }
}

#[derive(Debug, Gate, Back)]
pub struct BufferBind {
    pub slot: Hub<u32>,
    pub buffer: Hub<Grc<Buffer>>,
    pub offset: Option<Hub<u32>>,
}

impl Solve for BufferBind {
    type Base = stable::BufferBind;
    async fn solve(&self) -> node::Result<Self::Base> {
        let bind = stable::BufferBind {
            slot: self.slot.base().await?,
            buffer: self.buffer.base().await?,
            offset: self.offset.base().await?,
        };
        Ok(bind.into())
    }
}

#[derive(Debug, Gate, Back)]
pub struct Draw {
    pub stems: Vec<Hub<Grc<Action>>>,
    pub pipe: Hub<Grc<RenderPipeline>>,
    pub groups: Vec<Hub<stable::GroupBind>>,
    pub buffers: Vec<Hub<stable::BufferBind>>,
    pub vertex_offset: Hub<u32>,
    pub vertex_length: Hub<u32>,
    pub instance_offset: Hub<u32>,
    pub instance_length: Hub<u32>,
}

#[derive(Debug, Gate, Back)]
pub struct Dispatch {
    pub stems: Vec<Hub<Grc<Action>>>,
    pub pipe: Hub<Grc<ComputePipeline>>,
    pub binds: Vec<Hub<stable::GroupBind>>,
    pub size: Hub<u32>,
}

impl Solve for Dispatch {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let size = self.size.base().await?;
        let compute = action::tree::pass::Compute {
            pipe: self.pipe.base().await?,
            binds: self.binds.base().await?,
            kind: action::tree::pass::compute::Kind::Dispatch(size),
        };
        let action = action::tree::Action {
            stems: self.stems.base().await?,
            kind: action::tree::Kind::Compute(compute),
            ..Default::default()
        };
        Ok(Grc::new(action).into())
    }
}

impl Solve for Draw {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let vertex_offset = self.vertex_offset.base().await?;
        let vertex_end = vertex_offset + self.vertex_length.base().await?;
        let instance_offset = self.instance_offset.base().await?;
        let instance_end = instance_offset + self.instance_length.base().await?;
        let draw = stable::Draw {
            vertices: vertex_offset..vertex_end,
            instances: instance_offset..instance_end,
        };

        let render = action::tree::pass::Render {
            pipe: self.pipe.base().await?,
            groups: self.groups.base().await?,
            buffers: self.buffers.base().await?,
            kind: action::tree::pass::render::Kind::Draw(draw),
        };
        let action = action::tree::Action {
            stems: self.stems.base().await?,
            kind: action::tree::Kind::Render(render),
            ..Default::default()
        };
        Ok(Grc::new(action).into())
    }
}
