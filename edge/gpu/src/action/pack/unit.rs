use super::*;


#[derive(Debug, Back, Builder, BuildGate, Make)]
#[builder(pattern = "owned", setter(into))]
pub struct Dispatch {
    stems: Vec<Hub<Grc<Action>>>,
    pipe: Hub<Grc<ComputePipeline>>,
    #[builder(default, setter(each(name = "bind_inner", into)))]
    bindings: Vec<Hub<action::Binding>>,
    size: Hub<u32>,
}

impl Solve for Dispatch {
    type Base = Grc<Action>;
    async fn solve(&self) -> node::Result<Grc<Action>> {
        let dispatch = pack::Dispatch {
            stems: self.stems.base().await?,
            pipe: self.pipe.base().await?,
            bind: self.bindings.base().await?,
            size: self.size.base().await?,
        };
        Ok(Grc::new(Action::Dispatch(dispatch)).into())
    }
}

impl DispatchBuilder {
    pub fn bind(self, slot: impl Into<Hub<u32>>, group: impl Into<Hub<Grc<BindGroup>>>, offsets: impl Into<Hub<Vec<u32>>>) -> Self {
        self.bind_inner(BindingBuilder::default().slot(slot).group(group).offsets(offsets).hub().unwrap())
    }
}

#[derive(Debug, Clone, Back, Builder, BuildGate)]
#[builder(pattern = "owned")]
#[builder(setter(into))]
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

// self.bind_inner(Binding {slot: slot.into(), group: group.into(), offsets: offsets.into()}.hub().unwrap())

// bind: Vec<(Hub<u32>, Hub<Grc<BindGroup>>, Hub<Vec<u32>>)>,