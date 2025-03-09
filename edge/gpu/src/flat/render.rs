use super::*;

#[derive(Default, Debug)]
pub struct Pass {
    pub target: Target,
    pub steps: Vec<Step>,
}

#[derive(Default, Debug)]
pub enum Target {
    #[default]
    Frame,
    Texture(Vec<u8>),
}

#[derive(Debug)]
pub enum Step {
    Pipe(Grc<RenderPipeline>),
    Bind(stable::GroupBind),
    Vertex(stable::BufferBind),
    Index(Grc<Buffer>),
    Draw(command::draw::Direct),
    DrawIndexed(Range<u32>, i32, Range<u32>),
}

#[derive(Default)]
pub struct State {
    pipe: Option<Grc<RenderPipeline>>,
    binds: HashMap<u32, stable::GroupBind>,
    buffers: HashMap<u32, stable::BufferBind>,
    // index: Option<Grc<Buffer>>,
    steps: Vec<Step>,
}

impl State {
    pub fn flat(mut self) -> flat::Command {
        if let Some(pipe) = self.pipe {
            self.steps.push(Step::Pipe(pipe));
        }
        for (_, bind) in self.binds {
            self.steps.push(Step::Bind(bind));
        }
        for (_, buffer) in self.buffers {
            self.steps.push(Step::Vertex(buffer));
        }
        flat::Command::Render(Pass {
            steps: self.steps,
            ..Default::default()
        })
    }
    pub fn push(&mut self, render: &stable::command::Draw) {
        self.pipe(&render.pipe);
        self.binds(&render.groups);
        self.buffers(&render.buffers);
        self.draw(&render.kind);
    }
    pub fn pipe(&mut self, pipe: &Grc<RenderPipeline>) {
        if let Some(now) = self.pipe.as_mut() {
            if pipe != now { //if pipe.global_id() != now.global_id() {
                self.steps.push(Step::Pipe(now.clone()));
                *now = pipe.clone();
            }
        } else {
            self.pipe = Some(pipe.clone());
        }
    }
    pub fn binds(&mut self, binds: &[stable::GroupBind]) {
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
    fn buffers(&mut self, buffers: &[stable::BufferBind]) {
        for buffer in buffers {
            if let Some(now) = self.buffers.get_mut(&buffer.slot) {
                if buffer != now {
                    self.steps.push(Step::Vertex(now.clone()));
                    *now = buffer.clone();
                }
            } else {
                self.buffers.insert(buffer.slot, buffer.clone());
            }
        }
    }
    fn draw(&mut self, kind: &stable::command::draw::Kind) {
        match kind {
            stable::command::draw::Kind::Direct(draw) => self.steps.push(Step::Draw(draw.clone())),
            _ => panic!("crap"),
        }
    }
}
