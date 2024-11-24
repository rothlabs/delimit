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
        Pass { steps: self.steps }
    }
    pub fn push(&mut self, compute: &pack::pass::Compute) {
        self.pipe(&compute.pipe);
        self.binds(&compute.binds);
        self.dispatch(&compute.kind);
    }
    pub fn pipe(&mut self, pipe: &Grc<ComputePipeline>) {
        if let Some(now) = self.pipe.as_mut() {
            if !Grc::ptr_eq(pipe, now) {
                *now = pipe.clone();
                self.steps.push(Step::Pipe(pipe.clone()));
            }
        } else {
            self.pipe = Some(pipe.clone());
            self.steps.push(Step::Pipe(pipe.clone()));
        }
    }
    pub fn binds(&mut self, binds: &[Bind]) -> &mut Self {
        for bind in binds {
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
    fn dispatch(&mut self, kind: &pack::pass::compute::Kind) {
        match kind {
            pack::pass::compute::Kind::Dispatch(dispatch) => {
                self.steps.push(Step::Dispatch(*dispatch))
            }
            _ => panic!("crap"),
        }
    }
}
