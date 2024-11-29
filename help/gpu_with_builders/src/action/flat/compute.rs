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
    pub fn flat(mut self) -> Command {
        if let Some(pipe) = self.pipe {
            self.steps.push(Step::Pipe(pipe));
        }
        for (_, bind) in self.binds {
            self.steps.push(Step::Bind(bind));
        }
        Command::Compute(Pass { steps: self.steps })
    }
    pub fn push(&mut self, compute: &pack::pass::Compute) {
        self.pipe(&compute.pipe);
        self.binds(&compute.binds);
        self.dispatch(&compute.kind);
    }
    pub fn pipe(&mut self, pipe: &Grc<ComputePipeline>) {
        if let Some(now) = self.pipe.as_mut() {
            if pipe.global_id() != now.global_id() {
                self.steps.push(Step::Pipe(now.clone()));
                *now = pipe.clone();
            }
        } else {
            self.pipe = Some(pipe.clone());
        }
    }
    pub fn binds(&mut self, binds: &[Bind]) {
        for bind in binds {
            if let Some(now) = self.binds.get_mut(&bind.slot) {
                if bind != now {
                    self.steps.push(Step::Bind(now.clone()));
                    *now = bind.clone();
                }
            } else {
                self.binds.insert(bind.slot, bind.clone());
            }
        }
    }
    fn dispatch(&mut self, kind: &pack::pass::compute::Kind) {
        match kind {
            pack::pass::compute::Kind::Dispatch(dispatch) => {
                self.steps.push(Step::Dispatch(*dispatch))
            }
            _ => panic!("Indirect not implemented"),
        }
    }
}

// if let Some(now) = self.binds.get_mut(&bind.slot) {
//     if bind != now {
//         self.steps.push(Step::Bind(now.clone()));
//         *now = bind.clone();
//     }
// } else {
//     self.binds.insert(bind.slot, bind.clone());
// }
