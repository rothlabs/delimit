use super::*;

#[derive(Debug, Back, Builder, BuildGate)]
#[builder(pattern = "owned")]
pub struct Dispatch {
    stems: Vec<Hub<Grc<Action>>>,
    pipe: Hub<Grc<ComputePipeline>>,
    bind: Vec<Hub<Binding>>,
    size: Hub<u32>,
}

impl Solve for Dispatch {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let dispatch = pack::Dispatch {
            stems: self.stems.base().await?,
            pipe: self.pipe.base().await?,
            bind: vec![],
            size: self.size.base().await?,
        };
        Ok(Grc::new(Action::Dispatch(dispatch)).into())
    }
}

// impl ComputeBuilder {
//     pub fn pipe(self, pipe: Grc<ComputePipeline>) -> Self {
//         self.entry(Entry::Pipe(pipe))
//     }
//     pub fn bind(self, index: u32, bind: impl Into<Hub<Grc<BindGroup>>>) -> Self {
//         self.entry(Entry::Bind(index, bind.into()))
//     }
//     pub fn dispatch(self, count: impl Into<Hub<u32>>) -> Self {
//         self.entry(Entry::Dispatch(count.into()))
//     }
// }

 // , Builder, BuildGate
// #[builder(pattern = "owned")]
#[derive(Debug, Clone, Back)]
struct Binding {
    slot: Hub<u32>,
    group: Hub<Grc<BindGroup>>,
    offsets: Hub<Vec<u32>>,
}

impl Solve for Binding {
    type Base = action::Binding;
    async fn solve(&self) -> node::Result<action::Binding> {
        let binding = action::Binding {
            slot: self.slot.base().await?,
            group: self.group.base().await?,
            offsets: self.offsets.base().await?,
        };
        Ok(binding.into())
    }
}


// bind: Vec<(Hub<u32>, Hub<Grc<BindGroup>>, Hub<Vec<u32>>)>,