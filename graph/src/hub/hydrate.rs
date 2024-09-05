use super::*;

impl Hub {
    /// Build node connections in tree. A tree is a graph where each node has one root.
    /// This method traverses the tree to build additional connections using a name space system.
    pub fn hydrate(&self) -> Result<()> {
        let space = Space::new(vec![], self);
        self.saturate(&space, &space)?;
        Ok(())
    }
    fn saturate(&self, world: &Space, local: &Space) -> Result<()> {
        self.adapt(&mut Scope {
            world,
            local,
            back: None,
        })?;
        for space in local.map.values() {
            space.hub.saturate(world, space)?;
        }
        for space in &local.vec {
            space.hub.saturate(world, space)?;
        }
        Ok(())
    }
}
