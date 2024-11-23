use super::*;

#[derive(Default, Clone, Debug)]
pub struct Pass {
    pub steps: Vec<Step>,
}

impl Pass {
    pub fn push(&mut self, action: &Action) {
        // match action {
        //     Action::Draw(draw) => {
        //         self.steps
        //             .push(Step::Draw(draw.vertices.clone(), draw.instances.clone()));
        //     }
        //     _ => panic!("not render action"),
        // }
    }
}

#[derive(Clone, Debug)]
pub enum Step {
    Pipe(Grc<ComputePipeline>),
    Bind(u32, Grc<BindGroup>),
    Dispatch(u32),
}