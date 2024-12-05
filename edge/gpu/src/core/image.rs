use super::*;

pub struct Basic {
    pub stems: Vec<Hub<Grc<Action>>>,
    pub rig: Hub<action::Bind>,
    pub vertex_offset: Hub<u32>,
    pub vertex_length: Hub<u32>,
    pub instance_offset: Hub<u32>,
    pub instance_length: Hub<u32>,
}

impl<'a> Image<'a> {
    pub fn basic(self, basic: Basic) -> Hub<Grc<Action>> {
        let store = &self.core.store;
        Draw {
            pipe: self.pipe,
            buffers: vec![store.mesh.vertex.clone()],
            stems: basic.stems,
            binds: vec![basic.rig, store.topic.bind_vertex.clone()],
            vertex_offset: basic.vertex_offset,
            vertex_length: basic.vertex_length,
            instance_offset: basic.instance_offset,
            instance_length: basic.instance_length,
        }.hub()
    }
}
