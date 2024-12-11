use super::*;

pub mod dispatch {
    use super::*;
    #[derive(Debug, Gate, Back)]
    pub struct Direct {
        pub stems: Vec<Hub<Grc<Action>>>,
        pub pipe: Hub<Grc<ComputePipeline>>,
        pub binds: Vec<Hub<stable::GroupBind>>,
        pub size: Hub<u32>,
    }
    impl Solve for Direct {
        type Base = Grc<Action>;
        async fn solve(&self) -> node::Result<Grc<Action>> {
            let size = self.size.base().await?;
            let compute = action::tree::pass::Compute {
                pipe: self.pipe.base().await?,
                binds: self.binds.base().await?,
                kind: action::tree::pass::dispatch::Kind::Direct(size),
            };
            let action = action::tree::Action {
                stems: self.stems.base().await?,
                kind: action::tree::Kind::Compute(compute),
                ..Default::default()
            };
            Ok(Grc::new(action).into())
        }
    }
}

pub mod draw {
    use super::*;
    #[derive(Debug, Gate, Back)]
    pub struct Direct {
        pub stems: Vec<Hub<Grc<Action>>>,
        pub pipe: Hub<Grc<RenderPipeline>>,
        pub groups: Vec<Hub<stable::GroupBind>>,
        pub buffers: Vec<Hub<stable::BufferBind>>,
        pub vertex_offset: Hub<u32>,
        pub vertex_length: Hub<u32>,
        pub instance_offset: Hub<u32>,
        pub instance_length: Hub<u32>,
    }

    impl Solve for Direct {
        type Base = Grc<Action>;
        async fn solve(&self) -> node::Result<Grc<Action>> {
            let vertex_offset = self.vertex_offset.base().await?;
            let vertex_end = vertex_offset + self.vertex_length.base().await?;
            let instance_offset = self.instance_offset.base().await?;
            let instance_end = instance_offset + self.instance_length.base().await?;
            let draw = stable::command::draw::Direct {
                vertices: vertex_offset..vertex_end,
                instances: instance_offset..instance_end,
            };

            let render = action::tree::pass::Render {
                pipe: self.pipe.base().await?,
                groups: self.groups.base().await?,
                buffers: self.buffers.base().await?,
                kind: action::tree::pass::render::Kind::Direct(draw),
            };
            let action = action::tree::Action {
                stems: self.stems.base().await?,
                kind: action::tree::Kind::Render(render),
                ..Default::default()
            };
            Ok(Grc::new(action).into())
        }
    }
}
