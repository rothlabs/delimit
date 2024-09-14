use super::*;

#[derive(Default, Hash, Serialize, Deserialize, Debug)]
pub struct Bay {
    bay: u8,
    map: Map,
}

impl Bay {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, aim: impl Into<Aim>, apex: impl Into<Apex>) -> Result<()> {
        self.map.insert(aim, apex.into())?;
        Ok(())
    }
}

impl Adapt for Bay {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.map.deal(deal)?;
        Ok(())
    }
}

impl Solve for Bay {
    type Base = ();
    async fn solve(&self, task: Task<'_>) -> Result<Gain<()>> {
        match task {
            Task::Digest(state) => self.digest(state),
            Task::Serial => self.serial(),
            Task::React => solve_ok(),
            _ => task.no_handler(self),
        }
    }
}
