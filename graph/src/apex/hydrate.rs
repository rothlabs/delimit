use super::*;
use std::rc::Rc;

impl Apex {
    /// Build node connections in tree. A tree is a graph where each node has one root.
    /// This method traverses the tree to build additional connections using a name space system.
    pub fn hydrate(&self) -> Result<(), Error> {
        let scope = Scope::new(self)?;
        self.saturate(&scope)?;
        Ok(())
    }

    fn saturate(&self, scope: &Scope) -> Result<(), Error> {
        self.trade(scope);
        let root = Rc::new(scope.clone());
        for apex in self.stems()? {
            if let Ok(scope) = Scope::rooted(&root, &apex) {
                apex.saturate(&scope).ok();
            }
        }
        Ok(())
    }
}
