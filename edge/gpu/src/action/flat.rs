use super::*;
use std::collections::HashMap;
// use std::collections::HashSet;

pub mod render;

#[derive(Clone, Debug)]
pub enum Command {
    Compute,
    Render(render::Pass),
}

#[derive(Debug)]
pub struct Flat {
    actions: Vec<Hub<Grc<Action>>>,
    past: Leaf<HashMap<u64, Node>>,
}

impl Solve for Flat {
    type Base = Grc<Vec<Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let mut state = State {
            past: self.past.base()?,
            ..Default::default()
        };
        let actions = self.actions.base().await?;
        state.init(&actions);
        // state.commands(&actions);
        self.past.write_passive(|x| *x = state.nodes)?;
        Ok(Grc::new(state.commands).into())
    }
}

#[derive(Clone, Default)]
struct Node {
    need: u16,
    fill: u16,
    used: bool,
}

// impl Default for Node {
//     fn default() -> Self {
//         Self { need: 1, fill: (), used: () }
//     }
// }

#[derive(Default)]
struct State<'a> {
    actions: Vec<Grc<Action>>,
    past: HashMap<u64, Node>,
    nodes: HashMap<u64, Node>,
    // next: Vec<&'a Grc<Action>>,
    next: Vec<&'a Grc<Action>>,//HashMap<u64, &'a Grc<Action>>,
    commands: Vec<Command>,
}

impl<'a> State<'a> {
    fn init(&mut self, actions: &[Grc<Action>]) {
        let mut actions: Vec<&Grc<Action>> = self.actions.iter().collect();
        while let Some(action) = actions.pop() {
            let key = Grc::as_ptr(action) as u64;
            if let Some(node) = self.nodes.get_mut(&key) {
                node.need += 1;
            } else {
                self.nodes.insert(key, Node::default());
                if let Some(stems) = action.stems() {
                    actions.extend(stems);
                }
            }
        }
    }
    fn commands(&mut self) {
        let mut actions: Vec<&Grc<Action>> = self.actions.iter().collect();
        // self.next = self.actions.iter().collect();
        while !actions.is_empty() {
            
        }
    }
    fn try_action<F: FnOnce()>(
        &mut self,
        action: &'a Grc<Action>,
        use_action: F,
    ) -> Option<&'a [Grc<Action>]> {
        let key = Grc::as_ptr(action) as u64;
        if !self.past.contains_key(&key) {
            if let Some(node) = self.nodes.get_mut(&key) {
                if !node.used && node.fill >= node.need {
                    node.used = true;
                    use_action();
                    if let Some(stems) = action.stems() {
                        self.increment_node(stems);
                        return Some(stems);
                    }
                }
            }
        }
        None
    }
    fn increment_node(&mut self, actions: &[Grc<Action>]) {
        for action in actions {
            if let Some(node) = self.nodes.get_mut(&(Grc::as_ptr(action) as u64)) {
                node.fill += 1;
            }
        }
    }
}

struct Render<'a> {
    state: &'a mut State<'a>,
    pass: render::Pass,
}

impl<'a> Render<'a> {
    fn run(&mut self, actions: &'a [Grc<Action>]) {
        let mut actions: Vec<&Grc<Action>> = actions.iter().collect();
        while let Some(action) = actions.pop() {
            if let Some(Pass::Render) = action.pass() {
                if let Some(stems) = self.state.try_action(action, || self.pass.add(action)) {
                    actions.extend(stems);
                    continue;
                }
            }
            self.state.next.push(action);
        }
    }
}

// impl<'a> Render<'a> {
//     fn run(&mut self, actions: &'a [Grc<Action>], depth: u32) {
//         for action in actions {
//             if let Some(Pass::Render) = action.pass() {
//                 if let Some(stems) = self.state.try_action(action, || self.pass.add(action)) {
//                     self.run(stems, depth + 1);
//                     continue;
//                 }
//             }
//             self.state.next.push(action);
//         }
//     }
// }

// fn init(&mut self, actions: &[Grc<Action>], need: u16) {
//     for action in actions {
//         let key = Grc::as_ptr(action) as u64;
//         if let Some(node) = self.nodes.get_mut(&key) {
//             node.need += 1;
//         } else {
//             let node = Node {
//                 need,
//                 ..Default::default()
//             };
//             self.nodes.insert(key, node);
//             if let Some(stems) = action.stems() {
//                 self.init(stems, 1);
//             }
//         }
//     }
// }

// impl<'a> Render<'a> {
//     fn run(&mut self, actions: &[Grc<Action>]) {
//         for action in actions {
//             let key = Grc::as_ptr(action) as u64;
//             if let Some(Pass::Render) = action.pass() {
//                 if !self.state.past.contains_key(&key) {
//                     if let Some(node) = self.state.nodes.get_mut(&key) {
//                         if !node.used && node.fill >= node.need {
//                             node.used = true;
//                             self.pass.add(action);
//                             if let Some(stems) = action.stems() {
//                                 self.state.increase_node_fill(stems);
//                                 self.run(stems);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[derive(Default)]
// struct Field<T> {
//     base: T,
//     exclude: HashSet<u64>,
// }

// fn is_new(&self, action: &Grc<Action>) -> bool {
//     self.past.contains(&(Grc::as_ptr(action) as u64))
// }

// enum Slot {
//     Pass,
//     Pipe,
//     Bind(u32),
// }

// fn action_set(actions: &[Grc<Action>]) -> HashSet<u64> {
//     let mut set = HashSet::from_iter(actions.iter().map(|x| Grc::as_ptr(x) as u64));
//     for action in actions {
//         // if !set.contains(action )
//         if let Some(stems) = action.stems() {
//             set.extend(action_set(stems));
//         }
//     }
//     set
// }

// let set: HashSet<*const Action> = HashSet::from_iter(past.iter().map(Grc::as_ptr));

// // if let Some(action) = actions.first() {
// //     if let Some(mut state) = action.state() {
//         for action in &actions {
//             if !past.contains(&Grc::as_ptr(action)) {

//                 // let stems = action.stems();
//             }
//         }
//         let past = self.past.clone();
//         past.write_passive(|x| *x = actions)?;
// //     }
// // }

// #[derive(Default)]
// pub struct State {
//     pub pass: Pass,

// }

// #[derive(Default)]
// enum Pass {
//     #[default]
//     None,
//     Compute,
//     Render,
// }

// #[derive(Debug, Back, Gate)] // #[builder(pattern = "owned")]
// pub struct Buffer {
//     #[back(skip)]
//     viewport: Viewport,
//     commands: Hub<Vec<flat::Command>>,
// }

// impl Solve for Buffer {
//     type Base = Grc<CommandBuffer>;
//     async fn solve(&self) -> node::Result<Self::Base> {
//         let mut encoder = self.viewport.gpu.encoder();
//         let frame = self.viewport.frame()?;
//         let view = &frame.texture.create_view(&TextureViewDescriptor::default());
//         for command in &self.commands.base().await? {
//             if let flat::Command::Render(pass) = command {
//                 let attachments = self
//                     .viewport
//                     .gpu
//                     .attachment(&self.viewport.stage)
//                     .resolve_target(view)
//                     .list()?;
//                 let fields = &self.viewport.gpu.render_pass(&attachments).make()?;
//                 encoder.render(pass, fields);
//             }
//         }
//         Ok(Grc::new(encoder.finish()).into())
//     }
// }
