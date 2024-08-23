use super::*;

impl Apex {
    /// Build node connections in tree. A tree is a graph where each node has one root.
    /// This method traverses the tree to build additional connections using a name space system.
    pub fn hydrate(&self) {
        let space = Space::new(vec![], self);
        self.saturate(&space, &space);
    }

    fn saturate(&self, world: &Space, local: &Space) {
        self.trade(&Scope { world, local });
        for (_, next) in &local.map {
            next.apex.saturate(world, next);
        }
        for next in &local.vec {
            next.apex.saturate(world, next);
        }
    }
}
