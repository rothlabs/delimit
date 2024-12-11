use super::*;

pub struct Basic {
    pub stems: Vec<Hub<Grc<Action>>>,
    // pub rig: Hub<stable::GroupBind>,
    // pub vertex_offset: Hub<u32>,
    pub vertices: Hub<u32>,
    // pub instance_offset: Hub<u32>,
    pub instances: Hub<u32>,
}

// TODO: Gpu should not have Image. Mech or Medium should because Mech has the common gpu::active::group::Bind
impl<'a> Image<'a> {
    pub fn basic(&self, basic: Basic) -> Hub<Grc<Action>> {
        // let store = &self.core.store;
        // let wow = self.bind.topic;
        gpu::active::command::draw::Direct {
            pipe: self.medium.pipe.chart.points.clone().into(),
            // buffers: vec![store.mesh.vertex.clone()],
            buffers: vec![],
            stems: basic.stems,
            groups: vec![self.bind.clone(), self.medium.mech.group.bind.image.clone()], //, store.topic.bind_vertex.clone()],
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

// pub struct Basic {
//     pub stems: Vec<Hub<Grc<Action>>>,
//     pub rig: Hub<stable::GroupBind>,
//     // pub vertex_offset: Hub<u32>,
//     pub vertices: Hub<u32>,
//     // pub instance_offset: Hub<u32>,
//     pub instances: Hub<u32>,
// }

// // TODO: Gpu should not have Image. Mech or Medium should because Mech has the common gpu::active::group::Bind
// impl<'a> Image<'a> {
//     pub fn basic(self, basic: Basic) -> Hub<Grc<Action>> {
//         // let store = &self.core.store;
//         // let wow = self.bind.topic;
//         Draw {
//             pipe: self.pipe,
//             // buffers: vec![store.mesh.vertex.clone()],
//             buffers: vec![],
//             stems: basic.stems,
//             groups: vec![basic.rig, self.bind.topic],//, store.topic.bind_vertex.clone()],
//             vertex_offset: 0.into(), // basic.vertex_offset,
//             vertex_length: basic.vertices,
//             instance_offset: 0.into(), // basic.instance_offset,
//             instance_length: basic.instances,
//         }
//         .hub()
//     }
// }
