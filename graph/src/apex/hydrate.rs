use super::*;

impl Apex {
    /// Build node connections in tree. A tree is a graph where each node has one root.
    /// This method traverses the tree to build additional connections using a name space system.
    pub fn hydrate(&self) {
        let space = Space::new(vec![], self);
        self.saturate(&space, &space);
    }

    fn saturate(&self, world: &Space, local: &Space) {
        self.trade(&mut Scope { world, local });
        for space in local.map.values() {
            space.apex.saturate(world, space);
        }
        for space in &local.vec {
            space.apex.saturate(world, space);
        }
    }
}
