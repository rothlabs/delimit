use super::*;
use std::{collections::HashMap, rc::Rc};

#[derive(Default, Clone, Debug)]
pub struct Scope {
    vec: Vec<Apex>,
    map: HashMap<Key, Apex>,
    root: Option<Rc<Self>>,
}

impl Scope {
    pub fn new(apex: &Apex) -> Result<Self, Error> {
        let mut scope = Self::default();
        scope.vec.push(apex.clone());
        Ok(scope)
    }
    pub fn rooted(root: &Rc<Self>, apex: &Apex) -> Result<Self, Error> {
        let mut scope = Self::default();
        scope.root = Some(root.clone());
        scope.import(apex)?;
        Ok(scope)
    }
    fn import(&mut self, apex: &Apex) -> Result<(), Error> {
        if let Ok(imports) = apex.imports() {
            for import in &imports {
                match import {
                    Import::World(stem) => {
                        let scope = world(self.root.as_ref().ok_or("No root of scope.")?)?;
                        match stem {
                            meta::Stem::All => {
                                self.extend(scope.as_ref());
                            }
                            meta::Stem::Node(_) => {}
                        }
                    }
                    Import::Upper(upper) => {
                        let scope = self.upper(upper.rank)?;
                        match &upper.item {
                            meta::Stem::All => {
                                eprintln!("importing upper!");
                                eprintln!("scope: {:?}", scope.as_ref());
                                self.extend(scope.as_ref());
                            }
                            meta::Stem::Node(_) => {}
                        }
                    }
                    _ => (),
                }
            }
        }
        Ok(())
    }
    fn extend(&mut self, scope: &Scope) {
        self.vec.extend(scope.vec.clone());
        self.map.extend(scope.map.clone());
    }
    fn upper(&self, rank: usize) -> Result<Rc<Self>, Error> {
        let scope = self.root.as_ref().ok_or("No root of scope.")?;
        if rank > 1 {
            scope.upper(rank - 1)
        } else {
            Ok(scope.clone())
        }
    }
}

fn world(scope: &Rc<Scope>) -> Result<Rc<Scope>, Error> {
    if let Some(scope) = &scope.root {
        world(scope)
    } else {
        Ok(scope.clone())
    }
}

impl Trade for Scope {
    fn trade(&self, apex: &Apex) -> Apex {
        if let Apex::Tray(Tray::Path(Path::Local(local))) = apex {
            for apex in &self.vec {
                if let Ok(apex) = apex.get(local) {
                    return apex;
                }
            }
        }
        apex.clone()
    }
}
