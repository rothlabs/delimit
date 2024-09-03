use super::*;

impl Apex {
    /// Build node connections in tree. A tree is a graph where each node has one root.
    /// This method traverses the tree to build additional connections using a name space system.
    pub fn hydrate(&self) -> Result<Memo> {
        let space = Space::new(vec![], self);
        self.saturate(&space, &space)?;
        adapt_ok()
    }

    fn saturate(&self, world: &Space, local: &Space) -> Result<Memo> {
        self.adapt(&mut Scope { world, local, back: None })?;
        for space in local.map.values() {
            space.apex.saturate(world, space)?;
        }
        for space in &local.vec {
            space.apex.saturate(world, space)?;
        }
        adapt_ok()
    }
}
