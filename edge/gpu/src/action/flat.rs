use super::*;
use std::collections::HashSet;

pub mod render;

#[derive(Clone, Debug)]
pub enum Command {
    Compute,
    Render(render::Pass),
}

#[derive(Debug)]
pub struct Flat {
    actions: Vec<Hub<Grc<Action>>>,
    past: Leaf<HashSet<u64>>,
}

impl Solve for Flat {
    type Base = Grc<Vec<Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let state = State {
            past: self.past.base()?,
            ..Default::default()
        };
        let actions = self.actions.base().await?;
        let commands = state.commands(&actions);
        self.past.write_passive(|x| *x = action_set(&actions))?;
        Ok(commands)
    }
}

fn action_set(actions: &[Grc<Action>]) -> HashSet<u64> {
    let mut set = HashSet::new();
    for action in actions {
        let ptr = Grc::as_ptr(action) as u64;
        if !set.contains(&ptr) {
            set.insert(ptr);
            if let Some(stems) = action.stems() {
                set.extend(action_set(stems));
            }
        }
    }
    set
}

#[derive(Default)]
struct State {
    commands: Vec<Command>,
    past: HashSet<u64>,
    pass: Field<Option<Pass>>,
    // rank: u8,
    // action: &'a Grc<Action>,
}

impl State {
    fn include(&self, action: &Grc<Action>) -> bool {
        self.past.contains(&(Grc::as_ptr(action) as u64))
    }
    fn commands(mut self, actions: &[Grc<Action>]) -> Hub<Grc<Vec<Command>>> {
        // let actions = actions.iter().filter(|x| self.include(x));
        if let Some(Pass::Render) = self.pass.base {
            for action in actions {
                if !self.past.contains(&(Grc::as_ptr(action) as u64)) {
                    if let Some(Pass::Render) = action.pass() {
                        println!("wow");
                    } else if let Some(stems) = action.stems() {
                        let set: HashSet<u64> = HashSet::from_iter(stems.iter().map(|x| Grc::as_ptr(x) as u64));
                        self.pass.exclude.extend(set);
                    }
                }
            }
        }
        Grc::new(self.commands).into()
    }
}

#[derive(Default)]
struct Field<T> {
    base: T,
    exclude: HashSet<u64>,
}


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
