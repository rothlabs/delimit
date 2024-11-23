use std::collections::HashMap;

pub use pack::Action;

use super::*;

pub mod flat;
pub mod pack;

// // #[derive(Default)]
pub enum Pass {
    // #[default]
    Compute,
    Render,
}

#[derive(Debug, Clone)]
pub struct Bind {
    slot: u32,
    group: Grc<BindGroup>,
    offsets: Vec<u32>,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    slot: u32,
    buffer: Grc<Buffer>,
}

#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(pattern = "owned")]
pub struct Flat {
    actions: Vec<Hub<Grc<Action>>>,
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

#[derive(Clone, Default)]
struct Node {
    need: u16,
    fill: u16,
}

#[derive(Default)]
struct State<'a> {
    past: HashMap<u32, Node>,
    nodes: HashMap<u32, Node>,
    actions: [Vec<&'a Grc<Action>>; 2], 
    commands: Vec<Command>,
}

impl<'a> State<'a> {
    fn run(&mut self) {
        while let Some(action) = self.actions[1].pop() {
            let key = action.id;
            if let Some(node) = self.nodes.get_mut(&key) {
                node.need += 1;
            } else {
                self.nodes.insert(key, Node::default());
                self.actions[1].extend(&action.stems);
            }
        }
        self.actions[1].clear();
        self.passes();
    }
    fn passes(&mut self) {
        let mut i = (0, 1);
        while let Some(action) = self.actions[i.0].first() {
            match action.kind.pass() {
                Some(Pass::Render) => self.render(i),
                Some(Pass::Compute) => panic!("Pass::Compute not impl"),
                None => panic!("None not impl"),
            }
            self.actions[i.0].clear();
            i = (i.1, i.0);
        }
    }
    fn render(&mut self, i: (usize, usize)) {
        let mut pass = flat::render::Pass::default();
        while let Some(action) = self.actions[i.0].pop() {
            if let Some(Pass::Render) = action.kind.pass() {
                self.try_action(i.0, action, || pass.push(action));
            } else {
                self.actions[i.1].push(action);
            }
        }
        self.commands.push(Command::Render(pass));
    }
    fn try_action<F: FnOnce()>(&mut self, i: usize, action: &'a Grc<Action>, use_action: F) {
        if !self.past.contains_key(&action.id) {
            if let Some(node) = self.nodes.get(&action.id) {
                if node.fill == node.need {
                    use_action();
                    self.increment_stems(&action.stems);
                    self.actions[i].extend(&action.stems);
                }
            }
        }
    }
    fn increment_stems(&mut self, stems: &[Grc<Action>]) {
        for stem in stems {
            if let Some(node) = self.nodes.get_mut(&stem.id) {
                node.fill += 1;
            }
        }
    }
}











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
