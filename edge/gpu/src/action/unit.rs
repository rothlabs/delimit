use super::*;

#[derive(Debug, Gate, Back)]
pub struct Sort {
    actions: Vec<Hub<Grc<Action>>>,
    past: Leaf<HashMap<u32, Node>>,
}

impl Sort  {
    pub fn new(actions: impl Into<Vec<Hub<Grc<Action>>>>) -> Self {
        Self {
            actions: actions.into(),
            past: Leaf::default()
        }
    }
}

impl Solve for Sort {
    type Base = Grc<Vec<Command>>;
    async fn solve(&self) -> node::Result<Self::Base> {
        let actions = self.actions.base().await?;
        let actions: Vec<&Grc<Action>> = actions.iter().collect();
        let mut state = SortingState {
            actions: [actions.clone(), actions],
            past: self.past.base()?,
            ..Default::default()
        };
        state.sort();
        self.past.write_passive(|x| *x = state.nodes)?;
        Ok(Grc::new(state.commands).into())
    }
}

// #[derive(Debug, Back, Builder, BuildGate, Make)]
// #[builder(pattern = "owned")]

// #[builder(setter(each(name = "action")))]