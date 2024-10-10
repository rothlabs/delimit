use super::*;

#[derive(Default, Serialize, Deserialize, Debug)]
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

impl Solve for Bay {
    type Base = ();
    // fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
    //     self.map.deal(deal)?;
    //     Ok(())
    // }
}

impl Adapt for Bay {
    fn adapt(&mut self, deal: &mut dyn Deal) -> Result<()> {
        self.map.deal(deal)
    }
}

impl HashGraph for Bay {
    fn hash_graph<H: Hasher>(&self, state: &mut H) {
        self.bay.hash(state);
        self.map.hash_graph(state);
    }
}
