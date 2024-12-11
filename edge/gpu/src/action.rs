pub use tree::Action;
// pub use unit::Sort;

use super::*;
use std::collections::HashMap;

pub mod flat;
pub mod tree;

// mod unit;

#[derive(Debug, Gate, Back)]
pub struct Sort {
    actions: Vec<Hub<Grc<Action>>>,
    past: Leaf<HashMap<u32, Node>>,
}

impl Sort {
    pub fn new(actions: impl Into<Vec<Hub<Grc<Action>>>>) -> Self {
        Self {
            actions: actions.into(),
            past: Leaf::default(),
        }
    }
}

impl Solve for Sort {
    type Base = Grc<Vec<flat::Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let actions = self.actions.base().await?;
        let actions: Vec<&Grc<Action>> = actions.iter().collect();
        let mut state = SortingState {
            actions: [actions.clone(), actions],
            past: self.past.base()?,
            ..Default::default()
        };
        state.sort();
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
struct SortingState<'a> {
    past: HashMap<u32, Node>,
    nodes: HashMap<u32, Node>,
    actions: [Vec<&'a Grc<Action>>; 2],
    i: (usize, usize),
    commands: Vec<flat::Command>,
}

impl<'a> SortingState<'a> {
    fn sort(&mut self) {
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
                tree::Kind::Dispatch(_) => self.compute(),
                tree::Kind::Draw(_) => self.render(),
                tree::Kind::Leaf => panic!("should never be leaf here"),
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
                    tree::Kind::Dispatch(compute) => {
                        state.push(compute);
                        self.increment(&action.stems);
                    }
                    tree::Kind::Draw(_) => self.actions[self.i.1].push(action),
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
                    tree::Kind::Draw(render) => {
                        state.push(render);
                        self.increment(&action.stems);
                    }
                    tree::Kind::Dispatch(_) => self.actions[self.i.1].push(action),
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
