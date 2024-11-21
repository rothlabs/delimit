use super::*;
use std::collections::HashMap;
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
    past: Leaf<Vec<Grc<Action>>>,
}

impl Solve for Flat {
    type Base = Grc<Vec<Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let actions = self.actions.base().await?;
        let past = self.past.base()?;
        let past: HashSet<*const Action> = HashSet::from_iter(past.iter().map(Grc::as_ptr));
        let state = State {
            commands: vec![],
            past,
        };        
        Ok(state.commands(actions))
    }
}

struct State {
    commands: Vec<Command>,
    // actions: Vec<Grc<Action>>,
    past: HashSet<*const Action>,
}

impl State {
    fn commands(mut self, actions: Vec<Grc<Action>>) -> Hub<Grc<Vec<Command>>> {
        // for action in &actions {
        //     self.add_past(action);
        // }
        self.add_past(actions);
        Grc::new(self.commands).into()
    }
    fn add_past(&mut self, actions: Vec<Grc<Action>>) {
        for action in &actions {
            if let Some(stems) = action.stems() {
                
            }
            // for stem in &action.stems() {

            // }
            // self.add_past(action);
        }
    }
}

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
