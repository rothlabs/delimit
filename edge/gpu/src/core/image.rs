use super::*;

pub struct Basic {
    pub stems: Vec<Hub<Grc<Action>>>,
    pub rig: Hub<stable::GroupBind>,
    // pub vertex_offset: Hub<u32>,
    pub vertices: Hub<u32>,
    // pub instance_offset: Hub<u32>,
    pub instances: Hub<u32>,
}

impl<'a> Image<'a> {
    pub fn basic(self, basic: Basic) -> Hub<Grc<Action>> {
        // let store = &self.core.store;
        Draw {
            pipe: self.pipe,
            // buffers: vec![store.mesh.vertex.clone()],
            buffers: vec![],
            stems: basic.stems,
            groups: vec![basic.rig],//, store.topic.bind_vertex.clone()],
            vertex_offset: 0.into(), // basic.vertex_offset,
            vertex_length: basic.vertices,
            instance_offset: 0.into(), // basic.instance_offset,
            instance_length: basic.instances,
        }
        .hub()
    }
}

// local_count
// world_count
