use super::*;

#[derive(Default, Clone, Debug)]
pub struct Pass {
    pub steps: Vec<Step>,
}

#[derive(Clone, Debug)]
pub enum Step {
    Pipe(Grc<ComputePipeline>),
    Bind(Bind),
    Dispatch(u32),
}

#[derive(Default)]
pub struct State {
    pipe: Option<Grc<ComputePipeline>>,
    binds: HashMap<u32, Bind>,
    steps: Vec<Step>,
}

impl State {
    pub fn flat(self) -> Pass {
        Pass {
            steps: self.steps,
        }
    }
    pub fn pass(&mut self, pass: &pack::Pass) -> &mut Self {
        for bind in &pass.binds {
            if let Some(now) = self.binds.get_mut(&bind.slot) {
                if now != bind {
                    *now = bind.clone();
                    self.steps.push(Step::Bind(bind.clone()));
                }
            } else {
                self.binds.insert(bind.slot, bind.clone());
                self.steps.push(Step::Bind(bind.clone()));
            }
        }
        self
    }
}


