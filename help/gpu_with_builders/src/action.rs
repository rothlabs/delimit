pub use pack::Action;

use super::*;
use std::collections::HashMap;

pub mod flat;
pub mod pack;

pub enum Pass {
    Compute,
    Render,
}

#[derive(Debug, Clone)]
pub struct Bind {
    pub slot: u32,
    pub group: Grc<BindGroup>,
    pub offsets: Vec<u32>,
}

impl PartialEq for Bind {
    fn eq(&self, rhs: &Bind) -> bool {
        self.slot == rhs.slot
            && self.group.global_id() == rhs.group.global_id()
            && self.offsets == rhs.offsets
    }
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub slot: u32,
    pub buffer: Grc<Buffer>,
}

impl PartialEq for Vertex {
    fn eq(&self, rhs: &Vertex) -> bool {
        self.slot == rhs.slot && Grc::ptr_eq(&self.buffer, &rhs.buffer)
    }
}

#[derive(Debug, Clone)]
pub struct Draw {
    pub vertices: Range<u32>,
    pub instances: Range<u32>,
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(pattern = "owned")]
pub struct Flat {
    #[builder(setter(each(name = "action")))]
    actions: Vec<Hub<Grc<Action>>>,
    #[builder(default)]
    past: Leaf<HashMap<u32, Node>>,
}

impl Solve for Flat {
    type Base = Grc<Vec<Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let actions = self.actions.base().await?;
        let actions: Vec<&Grc<Action>> = actions.iter().collect();
        let mut state = State {
            actions: [actions.clone(), actions],
            past: self.past.base()?,
            ..Default::default()
        };
        state.run();
        self.past.write_passive(|x| *x = state.nodes)?;
        Ok(Grc::new(state.commands).into())
    }
}

#[derive(Clone)]
struct Node {
    need: u16,
    fill: u16,
}

#[derive(Default)]
struct State<'a> {
    past: HashMap<u32, Node>,
    nodes: HashMap<u32, Node>,
    actions: [Vec<&'a Grc<Action>>; 2],
    i: (usize, usize),
    commands: Vec<Command>,
}

impl<'a> State<'a> {
    fn run(&mut self) {
        self.init();
        self.actions[1].clear();
        self.i = (0, 1);
        self.passes();
    }
    fn init(&mut self) {
        while let Some(action) = self.actions[1].pop() {
            for stem in &action.stems {
                if let Some(node) = self.nodes.get_mut(&stem.id) {
                    node.need += 1;
                } else {
                    let node = Node { need: 1, fill: 0 };
                    self.nodes.insert(stem.id, node);
                    self.actions[1].push(stem);
                }
            }
        }
    }
    fn passes(&mut self) {
        while let Some(action) = self.actions[self.i.0].first() {
            match &action.kind {
                pack::Kind::Compute(_) => self.compute(),
                pack::Kind::Render(_) => self.render(),
                pack::Kind::Leaf => panic!("should never be leaf here"),
            }
            self.actions[self.i.0].clear();
            self.i = (self.i.1, self.i.0);
        }
    }
    fn compute(&mut self) {
        let mut state = flat::compute::State::default();
        while let Some(action) = self.actions[self.i.0].pop() {
            if !self.past.contains_key(&action.id) {
                match &action.kind {
                    pack::Kind::Compute(compute) => {
                        state.push(compute);
                        self.increment(&action.stems);
                    }
                    pack::Kind::Render(_) => self.actions[self.i.1].push(action),
                    _ => (),
                }
            }
        }
        self.commands.push(state.flat());
    }
    fn render(&mut self) {
        let mut state = flat::render::State::default();
        while let Some(action) = self.actions[self.i.0].pop() {
            if !self.past.contains_key(&action.id) {
                match &action.kind {
                    pack::Kind::Render(render) => {
                        state.push(render);
                        self.increment(&action.stems);
                    }
                    pack::Kind::Compute(_) => self.actions[self.i.1].push(action),
                    _ => (),
                }
            }
        }
        self.commands.push(state.flat());
    }
    fn increment(&mut self, stems: &'a [Grc<Action>]) {
        for stem in stems {
            if let Some(node) = self.nodes.get_mut(&stem.id) {
                node.fill += 1;
                if node.fill == node.need {
                    self.actions[self.i.0].push(stem);
                }
            }
        }
    }
}

// fn run(&mut self) {
//     self.i = (1, 0);
//     for action in &self.actions[1] {
//         let node = Node {need: 1, fill: 1};
//         self.nodes.insert(action.id, node);
//     }
//     while let Some(action) = self.actions[1].pop() {
//         // let key = action.id;
//         self.increment_need(&action.stems);
//         // self.extend(&action.stems);
//         for stem in &action.stems {
//             if !self.visited.contains(&stem.id) {
//                 self.visited.insert(stem.id);
//                 self.actions[self.i.0].push(stem);
//             }
//         }
//         // if let Some(node) = self.nodes.get_mut(&key) {
//         //     node.need += 1;
//         // } else {
//         //     self.nodes.insert(key, Node::default());
//         //     self.extend(&action.stems);
//         //     //self.actions[1].extend(&action.stems);
//         //     // self.increment_stems(&action.stems);
//         // }
//     }
//     self.actions[1].clear();
//     self.visited.clear();
//     self.passes();
// }
// fn increment_need(&mut self, stems: &[Grc<Action>]) {
//     for stem in stems {
//         if let Some(node) = self.nodes.get_mut(&stem.id) {
//             node.need += 1;
//         } else {
//             let node = Node {need: 1, fill: 0};
//             self.nodes.insert(stem.id, node);
//         }
//     }
// }
// // fn extend(&mut self, stems: &'a [Grc<Action>]) {
// //     for stem in stems {
// //         if !self.visited.contains(&stem.id) {
// //             self.visited.insert(stem.id);
// //             self.actions[self.i.0].push(stem);
// //         }
// //     }
// // }

// #[derive(Default)]
// struct State {
//     pass: Pass,
// }

// pub struct Segment {

//     commands: Vec<Command>
// }

// #[derive(Builder, Gate, Back, Debug)]
// #[builder(pattern = "owned")]
// #[builder(setter(into, strip_option))]
// pub struct Chain {
//     #[builder(default, setter(each(name = "pass", into)))]
//     passes: Vec<Pass>,
// }

// impl Solve for Chain {
//     type Base = Vec<Pass>;
//     async fn solve(&self) -> graph::Result<Hub<Vec<Pass>>> {
//         let passes = vec![];

//     }
// }

// impl Pass {
//     pub fn encode(&self, encoder: &mut Encode<'_>) {
//         let mut pass = encoder.compute();
//         for cmd in &self.pass.compute {
//             match cmd {
//                 compute::Entry::Pipe(pipe) => pass.set_pipeline(pipe),
//                 compute::Entry::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
//                 compute::Entry::Dispatch(count) => pass.dispatch_workgroups(*count, 1, 1),
//             }
//         }
//         let mut pass = encoder.render(self.descriptor);
//         for cmd in &self.pass.render {
//             match cmd {
//                 post::Render::Pipe(pipe) => pass.set_pipeline(pipe),
//                 post::Render::Bind(index, bind) => pass.set_bind_group(*index, bind, &[]),
//                 post::Render::Vertex(slot, buffer) => {
//                     pass.set_vertex_buffer(*slot, buffer.slice(..));
//                 }
//                 post::Render::Index(buffer) => {
//                     pass.set_index_buffer(buffer.slice(..), IndexFormat::Uint16);
//                 }
//                 post::Render::Draw(vertices, instances) => {
//                     pass.draw(vertices.clone(), instances.clone());
//                 }
//                 post::Render::DrawIndexed(indices, base_vertex, instances) => {
//                     pass.draw_indexed(indices.clone(), *base_vertex, instances.clone());
//                 }
//             }
//         }
//     }
// }
